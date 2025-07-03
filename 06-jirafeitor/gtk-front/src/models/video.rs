use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
}
