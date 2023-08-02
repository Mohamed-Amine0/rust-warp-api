use polars::prelude::*;
use std::{collections::HashMap, convert::Infallible};
use warp::Filter;
use std::iter::zip;

// Define a function to read the header of the CSV file and return it as JSON
async fn read_csv_header(file_path: String) -> Result<impl warp::Reply, Infallible> {
    // Read the CSV file and create a DataFrame
    let df = CsvReader::from_path(&file_path)
        .unwrap() //attempts to open and read the CSV file
        .has_header(true) //specify that the CSV file has a header row. This means that the first row of the CSV file contains column names, and the reader should interpret it as such
        .finish() //this method returns a Result containing either a DataFrame if successful or an error message if the CSV file cannot be parsed
        .unwrap(); //unwrap the Result to get the DataFrame

    // Return the header as a JSON response
    Ok(warp::reply::json(&df.get_column_names()))
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
        let row_data = df.get_row(row_index).unwrap().0;

        // Create a new vector to store the name-value
        let mut name_val = Vec::new();
        for (name, value) in zip(df.get_column_names().iter(), row_data.iter()) {
            name_val.push(format!("{}: {}", name, value));
        }

        // Return the JSON response
        Ok(warp::reply::json(&name_val))
    } else {
        // If the row_index is invalid, create an error message
        let error_response = format!("Row with index: {} not found", row_index);

        // Return the JSON response
        Ok(warp::reply::json(&error_response))
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
