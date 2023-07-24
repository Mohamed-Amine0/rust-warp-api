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
