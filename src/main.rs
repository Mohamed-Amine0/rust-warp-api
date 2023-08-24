mod handlers {
    pub mod column_handlers;
    pub mod row_handlers;
}
mod models;
mod utils;

use crate::handlers::column_handlers::*;
use crate::handlers::row_handlers::*;
use crate::utils::*;
use lazy_static::lazy_static;
use models::AddDataRequestBody;
use polars::prelude::*;
use warp::Filter;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("MISSING PATH");
        std::process::exit(1);
    }
    lazy_static! {
        static ref PARQUET_PATH: String = {
            let args: Vec<String> = std::env::args().collect();
            if args.len() < 2 {
                eprintln!("MISSING PATH");
                std::process::exit(1);
            }
            args[1].clone()
        };
    }

    let delete_column_route = warp::path!("delete-column" / String)
        .and(warp::delete())
        .and_then(|column_name: String| async move {
            delete_column_handler(
                PARQUET_PATH.to_string(),
                PARQUET_PATH.to_string(),
                column_name,
            )
            .await
        });

    let rename_column_route = warp::path!("rename-column" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(
            |old_column_name: String, new_column_data: Vec<String>| async move {
                rename_column_handler(
                    old_column_name,
                    PARQUET_PATH.to_string(),
                    PARQUET_PATH.to_string(),
                    new_column_data,
                )
                .await
            },
        );

    let add_data_to_column_route = warp::path!("add-data-to-column" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(
            |column_name: String, request_body: AddDataRequestBody| async move {
                add_data_to_column_handler(
                    PARQUET_PATH.to_string(),
                    PARQUET_PATH.to_string(),
                    column_name,
                    request_body,
                )
                .await
            },
        );

    let add_null_column_route = warp::path!("add-null-column" / String / String)
        .and(warp::post())
        .and_then(|column_name, column_type: String| async move {
            add_null_column_handler(
                PARQUET_PATH.to_string(),
                PARQUET_PATH.to_string(),
                column_name,
                column_type,
            )
            .await
        });

    let cast_column_route = warp::path!("cast-column" / String / String)
        .and(warp::put())
        .and_then(|column_name, new_type: String| async move {
            cast_column_handler(
                PARQUET_PATH.to_string(),
                PARQUET_PATH.to_string(),
                column_name,
                new_type,
            )
            .await
        });

    let add_column_with_values_route = warp::path!("add-column-with-values" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(
            |column_name: String, column_data: Vec<crate::AnyValue<'static>>| {
                let parquet_path = PARQUET_PATH.clone();
                let output_path = PARQUET_PATH.clone();
                async move {
                    add_column_with_values_handler(
                        parquet_path,
                        output_path,
                        column_name,
                        column_data,
                    )
                    .await
                }
            },
        );

    let add_row_route = warp::path!("add-row")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|new_row_values: Vec<AnyValue<'static>>| {
            let parquet_path = PARQUET_PATH.clone();
            let output_path = PARQUET_PATH.clone();
            async move { add_row_handler(parquet_path, output_path, new_row_values).await }
        });

    let delete_row_route = warp::path!("delete-row" / u32)
        .and(warp::delete())
        .and_then(move |row_index: u32| {
            delete_row_handler(
                PARQUET_PATH.to_string(),
                PARQUET_PATH.to_string(),
                row_index,
            )
        });

    let update_row_route = warp::path!("update-row" / u32)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(
            move |row_index: u32, new_row_values: Vec<AnyValue<'static>>| {
                update_row_handler(
                    PARQUET_PATH.to_string(),
                    PARQUET_PATH.to_string(),
                    row_index,
                    new_row_values,
                )
            },
        );

    let update_row_counter_route =
        warp::path!("update-row-counter")
            .and(warp::put())
            .and_then(move || {
                update_row_counter_handler(PARQUET_PATH.to_string(), PARQUET_PATH.to_string())
            });

    let display_dataframe_route =
        warp::path!("display-dataframe")
            .and(warp::get())
            .and_then(|| async move {
                let parquet_path = PARQUET_PATH.to_string();
                match read_parquet(&parquet_path).await {
                    Ok(df) => {
                        let df_json = serde_json::to_value(&df).unwrap();
                        Ok::<_, warp::Rejection>(warp::reply::json(&df_json))
                    }
                    Err(_) => Err(warp::reject::not_found()),
                }
            });

    let routes = delete_column_route
        .or(rename_column_route)
        .or(add_data_to_column_route)
        .or(add_null_column_route)
        .or(cast_column_route)
        .or(add_column_with_values_route)
        .or(add_row_route)
        .or(delete_row_route)
        .or(update_row_route)
        .or(update_row_counter_route)
        .or(display_dataframe_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
