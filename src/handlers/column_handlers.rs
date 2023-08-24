use crate::models::AddDataRequestBody;
use crate::utils::*;
use polars::prelude::*;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn delete_column_handler(
    input_path: String,
    output_path: String,
    column_name: String,
) -> Result<impl warp::Reply, Infallible> {
    let mut df = read_parquet(&input_path).await.unwrap();
    if df.get_column_names().contains(&column_name.as_str()) {
        df = delete_column(df, &column_name).await.unwrap();
        write_parquet(df.clone(), &output_path).await.unwrap();
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Column '{}' deleted successfully", column_name)),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Column '{}' not found", column_name)),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn rename_column_handler(
    old_column_name: String,
    input_path: String,
    output_path: String,
    new_column_data: Vec<String>,
) -> Result<impl warp::Reply, Infallible> {
    let mut df = read_parquet(&input_path).await.unwrap();
    if df.get_column_names().contains(&old_column_name.as_str()) {
        let new_column_name = new_column_data[0].clone();
        df = rename_column(df, &old_column_name, &new_column_name)
            .await
            .unwrap();
        write_parquet(df.clone(), &output_path).await.unwrap();
        Ok(warp::reply::with_status(
            warp::reply::json(&format!(
                "Column '{}' renamed to '{}'",
                old_column_name, new_column_name
            )),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Column '{}' not found", old_column_name)),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn add_data_to_column_handler(
    input_path: String,
    output_path: String,
    column_name: String,
    request_body: AddDataRequestBody,
) -> Result<impl warp::Reply, Infallible> {
    let df = match read_parquet(&input_path).await {
        Ok(df) => df,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error reading parquet file")),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    if !df.get_column_names().contains(&column_name.as_str()) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&format!("Column '{}' not found", column_name)),
            StatusCode::NOT_FOUND,
        ));
    }

    let data = request_body.data;
    let modified_df = match add_data_to_column(df, &column_name, data).await {
        Ok(modified_df) => modified_df,
        Err(err) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error: {}", err)),
                StatusCode::BAD_REQUEST,
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

pub async fn add_null_column_handler(
    input_path: String,
    output_path: String,
    column_name: String,
    column_type: String,
) -> Result<impl warp::Reply, Infallible> {
    let df = match read_parquet(&input_path).await {
        Ok(df) => df,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error reading parquet file")),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    if df.get_column_names().contains(&column_name.as_str()) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&format!("Column '{}' already exists", column_name)),
            StatusCode::NOT_FOUND,
        ));
    }

    let modified_df = match add_null_column(df, &column_name, &column_type).await {
        Ok(modified_df) => modified_df,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error adding null column")),
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

pub async fn cast_column_handler(
    parquet_path: String,
    output_path: String,
    column_name: String,
    new_type: String,
) -> Result<impl warp::Reply, Infallible> {
    let df = read_parquet(&parquet_path).await.unwrap();
    let modified_df = cast_column(df, &column_name, &new_type).await.unwrap();
    write_parquet(modified_df.clone(), &output_path)
        .await
        .unwrap();
    let df_json = serde_json::to_value(&modified_df).unwrap();
    Ok(warp::reply::json(&df_json))
}

pub async fn add_column_with_values_handler(
    parquet_path: String,
    output_path: String,
    column_name: String,
    column_data: Vec<AnyValue<'_>>,
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

    if column_data.is_empty() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&format!("Column data is missing")),
            StatusCode::BAD_REQUEST,
        ));
    }

    let modified_df = match add_column_with_values(df, &column_name, column_data).await {
        Ok(modified_df) => modified_df,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error adding column with values")),
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
