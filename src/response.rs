use crate::model::{Board, Dataset, User};
use serde::Serialize;

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