use std::{collections::HashMap, convert::Infallible};
use warp::Filter;
use polars::prelude::*;

// Define a function to read the header of the CSV file and return it as JSON
async fn read_csv_header(file_path: String) -> Result<impl warp::Reply, Infallible> {
    // Read the CSV file and create a DataFrame
    let df = CsvReader::from_path(&file_path)
        .unwrap() //attempts to open and read the CSV file
        .has_header(true) //specify that the CSV file has a header row. This means that the first row of the CSV file contains column names, and the reader should interpret it as such
        .finish() //this method returns a Result containing either a DataFrame if successful or an error message if the CSV file cannot be parsed
        .unwrap(); //unwrap the Result to get the DataFrame

    // Extract the header names from DataFrame columns and collect them into a Vec of Strings
    let header: Vec<String> = df
        .get_column_names()
        .iter()
        .map(|name| name.to_string())
        .collect();

    // Return the header as a JSON response
    Ok(warp::reply::json(&header))
}

// Define a function to read a row of the CSV file based on the given row_index and return it as JSON
async fn read_csv_row(file_path: String, row_index: usize) -> Result<impl warp::Reply, Infallible> {
    // Read the CSV file and create a DataFrame
    let df = CsvReader::from_path(&file_path)
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    // Check if the row_index is within the valid range of rows in the DataFrame
    if row_index < df.height() {
        // Get the data from the specified row
        let row_data = df.get_row(row_index).unwrap();

        // Convert each value to a String and handle null values
        let row_data_strings = row_data
            .0
            .iter()
            .map(|value| {
                if value.is_nested_null() {
                    "".to_string() // Replace null with an empty string
                } else {
                    // Check if the value is a string with double quotes and remove them
                    let val_str = value.to_string();
                    if val_str.starts_with('"') && val_str.ends_with('"') {
                        val_str[1..val_str.len() - 1].to_string()
                    } else {
                        val_str
                    }
                }
            })
            .collect::<Vec<String>>();

        // Return the JSON response
        Ok(warp::reply::json(&row_data_strings))
    } else {
        // If the row_index is invalid, create an error message
        let error_response = format!("Row with index: {} not found", row_index);

        // Serialize the error_response into a JSON response
        let serialized_response = serde_json::to_string(&error_response).unwrap();

        // Return the JSON response
        Ok(warp::reply::json(&serialized_response))
    }
}

#[tokio::main]
async fn main() {
    // Define routes for the health check
    let health_route = warp::path("health").and(warp::get()).map(|| "OK");

    // Define routes for the API
    let api_route_header = warp::path!("api" / "header")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(|params: HashMap<String, String>| {
            let file_path = params.get("file_path").cloned().unwrap_or_default();
            read_csv_header(file_path)
        });

    let api_route_row = warp::path!("api" / "row" / usize)
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(|row_index: usize, params: HashMap<String, String>| {
            let file_path = params.get("file_path").cloned().unwrap_or_default();
            read_csv_row(file_path, row_index)
        });

    // Combine the routes
    let routes = health_route.or(api_route_header).or(api_route_row);

    // Start the web server and run it on localhost:3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
