use polars::prelude::*;
use serde_json::Value;
use std::convert::Infallible;
use warp::Filter;

// Function to read the header of a CSV file and return it as JSON
async fn read_csv_header() -> Result<impl warp::Reply, Infallible> {
    // Read the CSV file and create a DataFrame
    let df = CsvReader::from_path("./src/titanic.csv")
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    // Extract the column names from the DataFrame and convert them to Strings
    let header: Vec<String> = df
        .get_columns()
        .iter()
        .map(|c| c.name().to_string())
        .collect();

    // Respond with the column names as JSON
    Ok(warp::reply::json(&header))
}

// Function to read a specific row from a CSV file and return it as JSON
async fn read_csv_row(row_index: usize) -> Result<impl warp::Reply, Infallible> {
    // Read the CSV file and create a DataFrame
    let df = CsvReader::from_path("./src/titanic.csv")
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    if row_index < df.height() {
        // If the requested row index exists, extract the row data and convert it to Strings
        let row_data = df
            .get_row(row_index)
            .unwrap()
            .0
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        // Serialize the row data and respond with it as JSON
        let serialized_response = serde_json::to_string(&row_data).unwrap();
        Ok(warp::reply::json(&serialized_response))
    } else {
        // If the requested row index does not exist, respond with an error message as JSON
        let error_response = format!("Row with index: {} not found", row_index);
        let serialized_response = serde_json::to_string(&error_response).unwrap();
        Ok(warp::reply::json(&serialized_response))
    }
}

// Function to delete a column from a CSV file and return a status message as JSON
async fn delete_csv_column(column_name: String) -> Result<impl warp::Reply, Infallible> {
    // Read the CSV file and create a DataFrame
    let mut df = CsvReader::from_path("./src/titanic.csv")
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    // Get the names of existing columns in the DataFrame
    let column_names: Vec<&str> = df.get_column_names().into_iter().map(|s| s).collect();

    if column_names.contains(&column_name.as_str()) {
        // If the column name exists in the DataFrame, drop the column and save the updated DataFrame
        let _ = df.drop_in_place(&column_name).unwrap();
        let mut file = std::fs::File::create("./src/titanic.csv").unwrap();
        CsvWriter::new(&mut file).finish(&mut df).unwrap();
        Ok(warp::reply::json(&format!(
            "Column '{}' deleted successfully",
            column_name
        )))
    } else {
        // If the column name does not exist in the DataFrame, respond with an error message as JSON
        Ok(warp::reply::json(&format!(
            "Column '{}' not found",
            column_name
        )))
    }
}

// Function to add a new column to a CSV file and return a status message as JSON
async fn add_csv_column<T: ToString + Send + Sync + 'static>(
    column_name: String,
    column_data: Vec<T>,
) -> Result<impl warp::Reply, Infallible> {
    // Convert column_data to Vec<String>
    let column_data_strings: Vec<String> = column_data.iter().map(|val| val.to_string()).collect();

    // Read the CSV file and create a DataFrame
    let mut df = CsvReader::from_path("./src/titanic.csv")
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    // Get the length of the provided column data and the number of rows in the DataFrame
    let data_column_length = column_data_strings.len();
    let df_height = df.height();

    if data_column_length < df_height {
        // If the provided column data is shorter than the DataFrame, pad it with null values for the missing rows
        let num_missing_rows = df_height - data_column_length;
        let mut padded_data = column_data_strings;
        padded_data.extend((0..num_missing_rows).map(|_| String::new()));

        // Create a new column with the padded data and add it to the DataFrame
        let data_column = Series::new(&column_name, padded_data);
        df.with_column(data_column).unwrap();
    } else if data_column_length == df_height {
        // If the provided column data has the same length as the DataFrame, create a new column with the data
        let data_column = Series::new(&column_name, column_data_strings);
        df.with_column(data_column).unwrap();
    } else {
        // If the provided column data is longer than the DataFrame, respond with an error message as JSON
        return Ok(warp::reply::json(&format!(
            "Data column '{}' has more data than the existing DataFrame",
            column_name
        )));
    }

    // Write the updated DataFrame to the CSV file
    let mut file = std::fs::File::create("./src/titanic.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    // Respond with a success message as JSON
    Ok(warp::reply::json(&format!(
        "Column '{}' added successfully",
        column_name
    )))
}

#[tokio::main]
async fn main() {
    // Read the CSV file and create a DataFrame
    let df = CsvReader::from_path("./src/titanic.csv")
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    // Print the DataFrame to the console
    println!("DataFrame df from titanic.csv:");
    println!("{:?}", df);
    // Define the health route that returns "OK" for health checks
    let health_route = warp::path("health").and(warp::get()).map(|| "OK");

    // Define the route to read the header of the CSV file
    let api_route_header = warp::path!("header")
        .and(warp::get())
        .and_then(read_csv_header);

    // Define the route to read a specific row from the CSV file
    let api_route_row = warp::path!("row" / usize)
        .and(warp::get())
        .and_then(read_csv_row);

    // Define the route to delete a column from the CSV file
    let api_route_delete_column = warp::path!("delete" / "column" / String)
        .and(warp::delete())
        .and_then(delete_csv_column);

    // Define the route to add a new column to the CSV file
    let api_route_add_column = warp::path!("add" / "column" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_csv_column::<Value>);

    // Combine all the defined routes
    let routes = health_route
        .or(api_route_header)
        .or(api_route_row)
        .or(api_route_delete_column)
        .or(api_route_add_column);

    // Start the server and listen on localhost:3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
