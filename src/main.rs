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
use std::env;
use std::fs;
use warp::Filter;

lazy_static! {
    static ref FILE_PATH: String = {
        let args: Vec<String> = env::args().collect();
        args.get(1).unwrap_or(&String::new()).to_string()
    };
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("MISSING PATH");
        std::process::exit(1);
    }

    let file_path = &args[1];
    if file_path.ends_with(".parquet")
    // || file_path.ends_with(".csv") for csv files support
    {
        if fs::metadata(file_path).is_ok() {
            file_path.to_string();
        } else {
            eprintln!("File doesn't exist!");
            std::process::exit(1);
        };
    } else {
        eprintln!("Invalid file format!");
        std::process::exit(1);
    }
    let delete_column_route = warp::path!("delete-column" / String)
        .and(warp::delete())
        .and_then(|column_name: String| async move {
            delete_column_handler(FILE_PATH.to_string(), FILE_PATH.to_string(), column_name).await
        });

    let rename_column_route = warp::path!("rename-column" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(
            |old_column_name: String, new_column_data: Vec<String>| async move {
                rename_column_handler(
                    old_column_name,
                    FILE_PATH.to_string(),
                    FILE_PATH.to_string(),
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
                    FILE_PATH.to_string(),
                    FILE_PATH.to_string(),
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
                FILE_PATH.to_string(),
                FILE_PATH.to_string(),
                column_name,
                column_type,
            )
            .await
        });

    let cast_column_route = warp::path!("cast-column" / String / String)
        .and(warp::put())
        .and_then(|column_name, new_type: String| async move {
            cast_column_handler(
                FILE_PATH.to_string(),
                FILE_PATH.to_string(),
                column_name,
                new_type,
            )
            .await
        });

    let add_column_with_values_route = warp::path!("add-column-with-values" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(
            |column_name: String, column_data: Vec<crate::AnyValue<'static>>| async move {
                add_column_with_values_handler(
                    FILE_PATH.clone(),
                    FILE_PATH.clone(),
                    column_name,
                    column_data,
                )
                .await
            },
        );

    let add_row_route = warp::path!("add-row")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|new_row_values: Vec<AnyValue<'static>>| async move {
            add_row_handler(FILE_PATH.clone(), FILE_PATH.clone(), new_row_values).await
        });

    let delete_row_route = warp::path!("delete-row" / u32)
        .and(warp::delete())
        .and_then(move |row_index: u32| {
            delete_row_handler(FILE_PATH.to_string(), FILE_PATH.to_string(), row_index)
        });

    let update_row_route = warp::path!("update-row" / u32)
        .and(warp::put())
        .and(warp::body::json())
        .and_then(
            move |row_index: u32, new_row_values: Vec<AnyValue<'static>>| {
                update_row_handler(
                    FILE_PATH.to_string(),
                    FILE_PATH.to_string(),
                    row_index,
                    new_row_values,
                )
            },
        );

    let update_row_counter_route = warp::path!("update-row-counter")
        .and(warp::put())
        .and_then(move || update_row_counter_handler(FILE_PATH.to_string(), FILE_PATH.to_string()));

    let display_dataframe_route =
        warp::path!("display-dataframe")
            .and(warp::get())
            .and_then(|| async move {
                match read_parquet(&FILE_PATH.to_string()).await {
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
