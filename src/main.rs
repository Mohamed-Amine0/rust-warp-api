
//model
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub id: Option<String>,
    pub name: String,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    pub id: Option<String>,
    pub name: String,
    pub board_id: Option<String>,
}


//response
//use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub status: String,
    pub data: User,
}

#[derive(Serialize)]
pub struct BoardResponse {
    pub status: String,
    pub data: Board,
}

#[derive(Serialize)]
pub struct DatasetResponse {
    pub status: String,
    pub data: Dataset,
}




//handler
/*
use crate::{
    model::{Board, Dataset, User},
    response::{GenericResponse, UserResponse, BoardResponse, DatasetResponse},
};
*/
//use crate::model::{Board, Dataset, User};
//use crate::response::{GenericResponse, UserResponse, BoardResponse, DatasetResponse};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json};

// Create a new user
//#[warp::post("/users")]
pub async fn create_user(user: User) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&UserResponse {
        status: "success".to_string(),
        data: user,
    }))
}

// Get a user by ID
//#[warp::get("/users/{id}")]
pub async fn get_user(id: Uuid) -> Result<impl warp::Reply, StatusCode> {
    let uuid_id = Uuid::new_v4();
    Ok(json(&UserResponse {
        status: "success".to_string(),
        data: User {
            id: Some(uuid_id.to_string()),
            name: "John Doe".to_string(),
        },
    }))
}

// Update a user by ID
//#[warp::put("/users/{id}")]
pub async fn update_user(id: Uuid, user: User) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&UserResponse {
        status: "success".to_string(),
        data: user,
    }))
}

// Delete a user by ID
//#[warp::delete("/users/{id}")]
pub async fn delete_user(id: Uuid) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&GenericResponse {
        status: "success".to_string(),
        message: "User deleted".to_string(),
    }))
}

// Create a new board
//#[warp::post("/boards")]
pub async fn create_board(board: Board) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&BoardResponse {
        status: "success".to_string(),
        data: board,
    }))
}

// Get a board by ID
//#[warp::get("/boards/{id}")]
pub async fn get_board(id: Uuid) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&BoardResponse {
        status: "success".to_string(),
        data: Board {
            id: Some(Uuid::new_v4().to_string()),
            name: "My Board".to_string(),
            user_id: Some(Uuid::new_v4().to_string()),
        },
    }))
}

// Update a board by ID
//#[warp::put("/boards/{id}")]
pub async fn update_board(id: Uuid, board: Board) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&BoardResponse {
        status: "success".to_string(),
        data: board,
    }))
}

// Delete a board by ID
//#[warp::delete("/boards/{id}")]
pub async fn delete_board(id: Uuid) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&GenericResponse {
        status: "success".to_string(),
        message: "Board deleted".to_string(),
    }))
}

// Create a new dataset
//#[warp::post("/datasets")]
pub async fn create_dataset(dataset: Dataset) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&DatasetResponse {
        status: "success".to_string(),
        data: dataset,
    }))
}

// Get a dataset by ID
//#[warp::get("/datasets/{id}")]
pub async fn get_dataset(id: Uuid) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&DatasetResponse {
        status: "success".to_string(),
        data: Dataset {
            id: Some(Uuid::new_v4().to_string()),
            name: "My Dataset".to_string(),
            board_id: Some(Uuid::new_v4().to_string()),
        },
    }))
}

// Update a dataset by ID
//#[warp::put("/datasets/{id}")]
pub async fn update_dataset(id: Uuid, dataset: Dataset) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&DatasetResponse {
        status: "success".to_string(),
        data: dataset,
    }))
}

// Delete a dataset by ID
//#[warp::delete("/datasets/{id}")]
pub async fn delete_dataset(id: Uuid) -> Result<impl warp::Reply, StatusCode> {
    Ok(json(&GenericResponse {
        status: "success".to_string(),
        message: "Dataset deleted".to_string(),
    }))
}

//main
use std::env;

use warp::{serve, Filter};
use warp::filters::log;


//mod handler;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("8080".to_string());

    let routes = warp::path("")
        .and(warp::get())
        .map(|| {
            format!("Welcome to the model API! You are running on port {}", port)
        });

    let api_routes = warp::path("api")
        .and(warp::path("users"))
        .and(warp::post())
        .and(warp::body::json())
        .map(create_user);

    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("users")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::get()))
            .map(get_user),
    );
/* 
    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("users")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::put()))
            .map(update_user),
    );
*/
    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("users")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::delete()))
            .map(delete_user),
    );

    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("boards"))
            .and(warp::post())
            .and(warp::body::json())
            .map(create_board),
    );

    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("boards")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::get()))
            .map(get_board),
    );
/* 
    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("boards")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::put()))
            .map(update_board),
    );
*/
    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("boards")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::delete()))
            .map(delete_board),
    );

    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("datasets"))
            .and(warp::post())
            .and(warp::body::json())
            .map(create_dataset),
    );

    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("datasets")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::get()))
            .map(get_dataset),
    );
/* 
    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("datasets")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::put()))
            .map(update_dataset),
    );
*/
    let api_routes = api_routes.or(
        warp::path("api")
            .and(warp::path("datasets")
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::delete()))
            .map(delete_dataset),
    );

    use warp::filters::reply::WithHeader;

    use warp::reply::json;

    let routes = routes.or(api_routes).with(warp::reply().with_header("Content-Type", "application/json").json(serde_json::json!({
        "message": "Hello, world!",
    })));
        
    serve(routes).run(([0, 0, 0, 0], port)).await;
}