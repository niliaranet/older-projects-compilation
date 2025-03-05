use crate::database::DatabaseWrapper;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Artist {
    #[schema(example = "artist name")]
    name: Option<String>,
    #[schema(example = "1")]
    id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ArtistPost {
    #[schema(example = "artist name", required = true)]
    name: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ArtistPut {
    #[schema(example = "1", required = true)]
    id: Option<String>,
    #[schema(example = "new name", required = true)]
    name: Option<String>,
}

impl DatabaseWrapper {
    pub async fn select_artists(&self) -> Result<Vec<Artist>, sqlx::Error> {
        sqlx::query_as!(
            Artist,
            "SELECT name, id
            FROM artist",
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn select_artist_by_id(&self, id: &str) -> Result<Option<Artist>, sqlx::Error> {
        sqlx::query_as!(
            Artist,
            "SELECT name, id
            FROM artist
            WHERE id = ?",
            id,
        )
        .fetch_optional(&self.db_pool)
        .await
    }

    pub async fn select_artists_by_name(&self, name_raw: &str) -> Result<Vec<Artist>, sqlx::Error> {
        let name: String = format!("{}{}{}", "%", name_raw, "%");
        sqlx::query_as!(
            Artist,
            "SELECT name, id
            FROM artist
            WHERE name LIKE ?",
            name,
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn create_artist(&self, data: ArtistPost) -> Result<MySqlQueryResult, sqlx::Error> {
        if data.name.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }
        sqlx::query!(
            "INSERT INTO artist (name)
            VALUE (?)",
            data.name
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn edit_artist(&self, data: ArtistPut) -> Result<MySqlQueryResult, sqlx::Error> {
        if data.id.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }
        let og_artist: Artist = match self.select_artist_by_id(data.id.as_ref().unwrap()).await {
            Ok(res) => match res.is_some() {
                true => res.unwrap(),
                false => return Err(sqlx::Error::RowNotFound),
            },
            Err(_) => return Err(sqlx::Error::RowNotFound),
        };
        sqlx::query!(
            "UPDATE artist SET name=? WHERE id=?",
            data.name
                .unwrap_or(og_artist.name.unwrap_or(String::default())),
            data.id,
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn delete_artist(&self, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        let _ = sqlx::query!(
            "DELETE song FROM song
            INNER JOIN album ON song.album_id = album.id
            WHERE album.artist_id = ?",
            id
        )
        .execute(&self.db_pool)
        .await;

        let _ = sqlx::query!(
            "DELETE FROM album
            WHERE artist_id = ?",
            id
        )
        .execute(&self.db_pool)
        .await;

        sqlx::query!(
            "DELETE FROM artist
            WHERE id = ?",
            id
        )
        .execute(&self.db_pool)
        .await
    }
}
