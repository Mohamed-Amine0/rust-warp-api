
//model.rs
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


//response.rs
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




//handler.rs
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