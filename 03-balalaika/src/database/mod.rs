pub mod album;
pub mod artist;
pub mod search_results;
pub mod song;
pub mod user;

use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Delete {
    #[schema(example = "1", required = true)]
    pub id: Option<String>,
}

pub struct DatabaseWrapper {
    db_pool: MySqlPool,
}

impl DatabaseWrapper {
    pub async fn new() -> Result<DatabaseWrapper, sqlx::Error> {
        let pool: MySqlPool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(env!("DATABASE_URL"))
            .await
            .unwrap(); /* This will break in case of error. It's intended. */

        Ok(DatabaseWrapper { db_pool: pool })
    }
}
