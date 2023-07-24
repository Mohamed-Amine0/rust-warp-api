use std::convert::Infallible;
use serde::Serialize;
use warp::{Filter, reply::{WithStatus, Json}};
use polars::prelude::*;
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};

// Define a function to read the header of the CSV file and return it as JSON
async fn read_csv_header() -> Result<impl warp::Reply, Infallible> {
    let df = CsvReader::from_path("/home/amine/Documents/internship/projects/wb_api/src/titanic.csv")
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();
    let header: Vec<String> = df.get_columns().iter().map(|c| c.name().to_string()).collect();

    Ok(warp::reply::json(&header))
}

// Define a struct for a generic response
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,                     // Status of the response
    pub message: String,                    // Message of the response
}




async fn read_csv_row(row_index: usize) -> Result<impl warp::Reply, Infallible> {
    let df = CsvReader::from_path("/home/amine/Documents/internship/projects/wb_api/src/titanic.csv")
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    // Vérifier si l'index de ligne est valide
    if row_index < df.height() {
        let row_data = df
            .get_row(row_index)
            .unwrap()
            .0.iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        let serialized_response = serde_json::to_string(&row_data).unwrap();

        Ok(warp::reply::json(&serialized_response))
    } else {
        // Si l'index de ligne est invalide, renvoyer une réponse d'erreur avec le code HTTP 404 (NOT_FOUND)
        // Wrap the JSON response with the status code
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Row with index: {} not found", row_index),
        };

        let serialized_response = serde_json::to_string(&error_response).unwrap();

        Ok(warp::reply::json(&serialized_response))
    }
}



#[tokio::main]
async fn main() {
    let api_route_header = warp::path("api")
        .and(warp::path("header"))
        .and(warp::get())
        .and_then(read_csv_header);

    let api_route_row = warp::path!("api" / "row" / usize)
        .and(warp::get())
        .and_then(read_csv_row);

    let routes = api_route_header.or(api_route_row);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
