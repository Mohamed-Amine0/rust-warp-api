use std::convert::Infallible;
use warp::Filter;
use polars::prelude::*;

// Define a function to read the header of the CSV file and return it as JSON
async fn read_csv_header() -> Result<impl warp::Reply, Infallible> {
    // Read the CSV file and create a DataFrame
    let df = CsvReader::from_path("/home/amine/Documents/internship/projects/wb_api/src/titanic.csv")
        .unwrap() //attempts to open and read the CSV file
        .has_header(true) //specify that the CSV file has a header row. This means that the first row of the CSV file contains column names, and the reader should interpret it as such
        .finish() //this method returns a Result containing either a DataFrame if successful or an error message if the CSV file cannot be parsed
        .unwrap(); //unwrap the Result to get the DataFrame

    // Extract the header names from DataFrame columns and collect them into a Vec of Strings
    let header: Vec<String> = df.get_columns() //get the columns of the DataFrame
                                .iter() //iterate over the columns
                                .map(|c| c.name().to_string()) //get the name of each column and convert it to a String
                                .collect(); //collect the Strings into a Vec

    // Return the header as a JSON response
    Ok(warp::reply::json(&header))
}

// Define a function to read a row of the CSV file based on the given row_index and return it as JSON
async fn read_csv_row(row_index: usize) -> Result<impl warp::Reply, Infallible> {
    // Read the CSV file and create a DataFrame
    let df = CsvReader::from_path("/home/amine/Documents/internship/projects/wb_api/src/titanic.csv")
        .unwrap() 
        .has_header(true)
        .finish()
        .unwrap();

    // Check if the row_index is within the valid range of rows in the DataFrame
    if row_index < df.height() {
        // Get the data from the specified row and convert it to a Vec of Strings
        let row_data = df
            .get_row(row_index) //get the row at the specified index
            .unwrap() //unwrap the Result to get the Row
            .0.iter() //iterate over the values in the Row
            .map(|value| value.to_string()) //convert each value to a String
            .collect::<Vec<String>>();

        // Serialize the row_data into a JSON response
        let serialized_response = serde_json::to_string(&row_data).unwrap();

        // Return the JSON response
        Ok(warp::reply::json(&serialized_response))
    } else {
        // If the row_index is invalid, create an error message
        let error_response = format!("Row with index: {} not found", row_index);

        // Serialize the error_response into a JSON response
        let serialized_response = serde_json::to_string(&error_response).unwrap();

        // Return the JSON response
        Ok(warp::reply::json(&serialized_response))
    }
}

// Define a function to get the type of a variable
fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}



#[tokio::main]
async fn main() {


    let df = CsvReader::from_path("/home/amine/Documents/internship/projects/wb_api/src/titanic.csv")
        .unwrap() 
        .has_header(true)
        .finish()
        .unwrap();

        // Get the data from the specified row and convert it to a Vec of Strings
        let row_data = df
            .get_row(3) //get the row at the specified index
            .unwrap() //unwrap the Result to get the Row
            .0.iter() //iterate over the values in the Row
            .map(|value| value.to_string()) //convert each value to a String
            .collect::<Vec<String>>();

    // Print the result of the df.get_row(3).unwrap() method call
    println!("\n\n\n\n\n\nTHIS IS THE RESULT OF df.get_row(3).unwrap() METHOD CALL:\n");
    println!("{:?}", df.get_row(3).unwrap());  
    // Print the type of the df.get_row(3).unwrap() method call
    println!("\n\nTHIS IS THE TYPE OF df.get_row(3).unwrap() METHOD CALL:\n");
    println!("{:?}", type_of(df.get_row(3).unwrap()));

    // Print the result of the df.get_row(3).unwrap().0 method call
    println!("\n\n\n\n\n\nTHIS IS THE RESULT OF df.get_row(3).unwrap().0 METHOD CALL:\n");
    println!("{:?}", df.get_row(3).unwrap().0);
    // Print the type of the df.get_row(3).unwrap().0 method call
    println!("\n\nTHIS IS THE TYPE OF df.get_row(3).unwrap().0 METHOD CALL:\n");
    println!("{:?}", type_of(df.get_row(3).unwrap().0));

    // Print the result of the df.get_rows(3).unwrap().0.iter() method call
    println!("\n\n\n\n\n\nTHIS IS THE RESULT OF df.get_rows(3).unwrap().0.iter() METHOD CALL:\n");
    println!("{:?}", df.get_row(3).unwrap().0.iter());
    // Print the type of the df.get_rows(3).unwrap().0.iter() method call
    println!("\n\nTHIS IS THE TYPE OF df.get_rows(3).unwrap().0.iter() METHOD CALL:\n");
    println!("{:?}", type_of(df.get_row(3).unwrap().0.iter()));

    // Print row_data
    println!("\n\n\n\n\n\nTHIS IS THE ROW DATA:\n");
    println!("{:?}", row_data);
    // Print the type of row_data
    println!("\n\nTHIS IS THE TYPE OF ROW DATA:\n");
    println!("{:?}", type_of(row_data));

    // Print the df
    println!("\n\n\n\n\n\nTHIS IS THE DATAFRAME:\n");
    println!("{:?}", df);
    // Print the type of the df
    println!("\n\nTHIS IS THE TYPE OF DATAFRAME:\n");
    println!("{:?}", type_of(df));

    


    // Print the result of the df.get_columns() method call
    //println!("\n\n\n\n\n\nTHIS IS THE RESULT OF df.get_columns() METHOD CALL:\n\n");
    //println!("{:?}", df.get_columns());


    
    // Define routes for the health check
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| "OK");
    

    // Define routes for the API
    let api_route_header = warp::path!("api" /"header")
        .and(warp::get())
        .and_then(read_csv_header);

    let api_route_row = warp::path!("api" / "row" / usize)
        .and(warp::get())
        .and_then(read_csv_row);

    // Combine the routes
    let routes = health_route
        .or(api_route_header)
        .or(api_route_row);

        
    // Start the web server and run it on localhost:3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}