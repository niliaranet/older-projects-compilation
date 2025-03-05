use crate::database::DatabaseWrapper;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Album {
    #[schema(example = "album name")]
    name: Option<String>,
    #[schema(example = "1")]
    id: Option<i32>,
    #[schema(example = "just ignore this one")]
    artist_name: Option<String>,
    #[schema(example = "1")]
    artist_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AlbumPost {
    #[schema(example = "album name")]
    name: Option<String>,
    #[schema(example = "just ignore this one")]
    artist_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AlbumPut {
    id: Option<String>,
    #[schema(example = "album name")]
    name: Option<String>,
    #[schema(example = "just ignore this one")]
    artist_id: Option<String>,
}

impl DatabaseWrapper {
    pub async fn select_albums(&self) -> Result<Vec<Album>, sqlx::Error> {
        sqlx::query_as!(
            Album,
            "SELECT album.name, album.id,
            artist.name as artist_name, artist.id as artist_id
            FROM album
            INNER JOIN artist ON album.artist_id = artist.id
            ",
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn select_album_by_id(&self, id: &str) -> Result<Option<Album>, sqlx::Error> {
        sqlx::query_as!(
            Album,
            "SELECT album.name, album.id,
            artist.name as artist_name, artist.id as artist_id
            FROM album
            INNER JOIN artist ON album.artist_id = artist.id
            WHERE album.id=?",
            id,
        )
        .fetch_optional(&self.db_pool)
        .await
    }

    pub async fn select_albums_by_name(&self, name_raw: &str) -> Result<Vec<Album>, sqlx::Error> {
        let name: String = format!("{}{}{}", "%", name_raw, "%");
        sqlx::query_as!(
            Album,
            "SELECT album.name, album.id,
            artist.name as artist_name, artist.id as artist_id
            FROM album
            INNER JOIN artist ON album.artist_id = artist.id
            WHERE LOWER(album.name) LIKE LOWER(?)
            ",
            name,
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn select_albums_by_artist(
        &self,
        artist_id: &str,
    ) -> Result<Vec<Album>, sqlx::Error> {
        sqlx::query_as!(
            Album,
            "SELECT album.name, album.id,
            artist.name as artist_name, artist.id as artist_id
            FROM album
            INNER JOIN artist ON album.artist_id = artist.id
            WHERE artist.id=?",
            artist_id,
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn create_album(&self, data: AlbumPost) -> Result<MySqlQueryResult, sqlx::Error> {
        if data.name.is_none() || data.artist_id.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        if match self
            .select_artist_by_id(data.artist_id.as_ref().unwrap())
            .await
        {
            Ok(res) => res.is_none(),
            Err(_) => true,
        } {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            "INSERT INTO album (name, artist_id)
            VALUE (?, ?)",
            data.name,
            data.artist_id,
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn edit_album(&self, data: AlbumPut) -> Result<MySqlQueryResult, sqlx::Error> {
        if data.id.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }
        let og_album: Album = match self.select_album_by_id(data.id.as_ref().unwrap()).await {
            Ok(res) => match res.is_some() {
                true => res.unwrap(),
                false => return Err(sqlx::Error::RowNotFound),
            },
            Err(_) => return Err(sqlx::Error::RowNotFound),
        };
        sqlx::query!(
            "UPDATE album SET name=? WHERE id=?",
            data.name
                .unwrap_or(og_album.name.unwrap_or(String::default())),
            data.id,
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn delete_album(&self, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        let _ = sqlx::query!(
            "DELETE FROM song
            WHERE album_id = ?",
            id
        )
        .execute(&self.db_pool)
        .await;

        sqlx::query!(
            "DELETE FROM album
            WHERE id = ?",
            id
        )
        .execute(&self.db_pool)
        .await
    }
}
