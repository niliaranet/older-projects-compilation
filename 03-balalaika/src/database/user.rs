use crate::database::DatabaseWrapper;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: Option<String>,
    password: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserForm {
    #[schema(example = "username")]
    name: Option<String>,
    #[schema(example = "password")]
    password: Option<String>,
}

impl DatabaseWrapper {
    pub async fn register(&self, data: UserForm) -> Result<MySqlQueryResult, sqlx::Error> {
        if data.name.is_none() || data.password.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            "INSERT INTO user (name, password)
            VALUE (?, PASSWORD(?))",
            data.name,
            data.password,
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn login(&self, data: UserForm) -> Result<Option<User>, sqlx::Error> {
        if data.name.is_none() || data.password.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query_as!(
            User,
            "SELECT * FROM user
            WHERE ? = name
            AND password(?) = password
            ",
            data.name,
            data.password,
        )
        .fetch_optional(&self.db_pool)
        .await
    }

    pub async fn delete_user(&self, data: UserForm) -> Result<MySqlQueryResult, sqlx::Error> {
        if data.name.is_none() || data.password.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        match sqlx::query_as!(
            User,
            "SELECT * FROM user
            WHERE name = ?
            AND password = password(?)
            ",
            data.name,
            data.password,
        )
        .fetch_one(&self.db_pool)
        .await
        {
            Ok(_) => (),
            Err(_) => return Err(sqlx::Error::RowNotFound),
        };

        sqlx::query!(
            "DELETE FROM user
            WHERE name = ?
            AND password = password(?)
            ",
            data.name,
            data.password,
        )
        .execute(&self.db_pool)
        .await
    }
}
