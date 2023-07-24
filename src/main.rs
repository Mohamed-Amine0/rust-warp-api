use std::convert::Infallible;
use warp::Filter;
use polars::prelude::*;

// Define a function to read the header of the CSV file and return it as JSON
async fn read_csv_header() -> Result<impl warp::Reply, Infallible> {
    //let file_path = "titanic.csv";
    let df = CsvReader::from_path("/home/amine/Documents/internship/projects/wb_api/src/titanic.csv")
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();
    let header: Vec<String> = df.get_columns().iter().map(|c| c.name().to_string()).collect();

    Ok(warp::reply::json(&header))
}

#[tokio::main]
async fn main() {
    let api_route = warp::path!("api" / "header")
        .and(warp::get())
        .and_then(read_csv_header);

    warp::serve(api_route).run(([127, 0, 0, 1], 3030)).await;
}
