use crate::utils::*;
use polars::prelude::*;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn add_row_handler(
    parquet_path: String,
    output_path: String,
    new_row_values: Vec<AnyValue<'_>>,
) -> Result<impl warp::Reply, Infallible> {
    let df = match read_parquet(&parquet_path).await {
        Ok(df) => df,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error reading parquet file")),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    if new_row_values.is_empty() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&format!("New row values are missing")),
            StatusCode::BAD_REQUEST,
        ));
    }

    let modified_df = match add_row(df, new_row_values).await {
        Ok(modified_df) => modified_df,
        Err(err) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error adding row: {}", err)),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    if let Err(_) = write_parquet(modified_df.clone(), &output_path).await {
        return Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error writing parquet file")),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let df_json = match serde_json::to_value(&modified_df) {
        Ok(df_json) => df_json,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error serializing DataFrame to JSON")),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&df_json),
        StatusCode::OK,
    ))
}

pub async fn delete_row_handler(
    parquet_path: String,
    output_path: String,
    row_index: u32,
) -> Result<impl warp::Reply, Infallible> {
    let mut df = read_parquet(&parquet_path).await.unwrap();
    if row_index < df.height().try_into().unwrap() {
        df = delete_row(df, row_index).await.unwrap();
        write_parquet(df.clone(), &output_path).await.unwrap();
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Row at index {} deleted successfully", row_index)),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Row at index {} not found", row_index)),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn update_row_handler(
    parquet_path: String,
    output_path: String,
    row_index: u32,
    new_row_values: Vec<AnyValue<'_>>,
) -> Result<impl warp::Reply, Infallible> {
    let df = match read_parquet(&parquet_path).await {
        Ok(df) => df,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error reading parquet file")),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    if row_index >= df.height().try_into().unwrap() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&format!("Row index out of bounds")),
            StatusCode::NOT_FOUND,
        ));
    }

    let modified_df = match update_row(df, row_index, new_row_values).await {
        Ok(modified_df) => modified_df,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error updating row")),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    if let Err(_) = write_parquet(modified_df.clone(), &output_path).await {
        return Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error writing parquet file")),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let df_json = match serde_json::to_value(&modified_df) {
        Ok(df_json) => df_json,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error serializing DataFrame to JSON")),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&df_json),
        StatusCode::OK,
    ))
}

pub async fn update_row_counter_handler(
    parquet_path: String,
    output_path: String,
) -> Result<impl warp::Reply, Infallible> {
    let df = read_parquet(&parquet_path).await.unwrap();
    let modified_df = update_row_counter(df).await.unwrap();
    write_parquet(modified_df.clone(), &output_path)
        .await
        .unwrap();
    let df_json = serde_json::to_value(&modified_df).unwrap();
    Ok(warp::reply::json(&df_json))
}
