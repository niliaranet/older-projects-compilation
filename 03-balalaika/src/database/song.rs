use crate::database::DatabaseWrapper;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Song {
    #[schema(example = "song name")]
    name: Option<String>,
    #[schema(example = "1")]
    id: Option<i32>,
    #[schema(example = "song lyrics...")]
    lyrics: Option<String>,
    #[schema(example = "album name")]
    album_name: Option<String>,
    #[schema(example = "1")]
    album_id: Option<i32>,
    #[schema(example = "artist name")]
    artist_name: Option<String>,
    #[schema(example = "1")]
    artist_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SongPost {
    #[schema(example = "song name")]
    name: Option<String>,
    #[schema(example = "song lyrics...")]
    lyrics: Option<String>,
    #[schema(example = "1")]
    album_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SongPut {
    #[schema(example = "1")]
    id: Option<String>,
    #[schema(example = "song name")]
    name: Option<String>,
    #[schema(example = "song lyrics...")]
    lyrics: Option<String>,
    #[schema(example = "1")]
    album_id: Option<String>,
}

impl DatabaseWrapper {
    pub async fn select_songs(&self) -> Result<Vec<Song>, sqlx::Error> {
        sqlx::query_as!(
            Song,
            "SELECT song.name, song.lyrics, song.id,
            album.name as album_name, album.id as album_id,
            artist.name as artist_name, artist.id as artist_id
            FROM song
            INNER JOIN album ON song.album_id = album.id
            INNER JOIN artist ON album.artist_id = artist.id
            ",
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn select_song_by_id(&self, id: &str) -> Result<Option<Song>, sqlx::Error> {
        sqlx::query_as!(
            Song,
            "SELECT song.name, song.lyrics, song.id,
            album.name as album_name, album.id as album_id,
            artist.name as artist_name, artist.id as artist_id
            FROM song
            INNER JOIN album ON song.album_id = album.id
            INNER JOIN artist ON album.artist_id = artist.id
            WHERE song.id = ?
            ",
            id,
        )
        .fetch_optional(&self.db_pool)
        .await
    }

    pub async fn select_songs_by_album(&self, album_id: &str) -> Result<Vec<Song>, sqlx::Error> {
        sqlx::query_as!(
            Song,
            "SELECT song.name, song.lyrics, song.id,
            album.name as album_name, album.id as album_id,
            artist.name as artist_name, artist.id as artist_id
            FROM song
            INNER JOIN album ON song.album_id = album.id
            INNER JOIN artist ON album.artist_id = artist.id
            WHERE album.id = ?
            ",
            album_id,
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn select_songs_by_artist(&self, artist_id: &str) -> Result<Vec<Song>, sqlx::Error> {
        sqlx::query_as!(
            Song,
            "SELECT song.name, song.lyrics, song.id,
            album.name as album_name, album.id as album_id,
            artist.name as artist_name, artist.id as artist_id
            FROM song
            INNER JOIN album ON song.album_id = album.id
            INNER JOIN artist ON album.artist_id = artist.id
            WHERE artist.id = ?
            ",
            artist_id,
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn select_songs_by_name(&self, name_raw: &str) -> Result<Vec<Song>, sqlx::Error> {
        let name: String = format!("{}{}{}", "%", name_raw, "%");
        sqlx::query_as!(
            Song,
            "SELECT song.name, song.lyrics, song.id,
            album.name as album_name, album.id as album_id,
            artist.name as artist_name, artist.id as artist_id
            FROM song
            INNER JOIN album ON song.album_id = album.id
            INNER JOIN artist ON album.artist_id = artist.id
            WHERE LOWER(song.name) LIKE LOWER(?)",
            name,
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn create_song(&self, data: SongPost) -> Result<MySqlQueryResult, sqlx::Error> {
        if data.name.is_none() || data.album_id.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        if match self
            .select_album_by_id(data.album_id.as_ref().unwrap())
            .await
        {
            Ok(res) => res.is_none(),
            Err(_) => true,
        } {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query!(
            "INSERT INTO song (name, lyrics, album_id)
            VALUE (?, ?, ?)",
            data.name,
            data.lyrics.unwrap_or(String::default()),
            data.album_id,
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn edit_song(&self, data: SongPut) -> Result<MySqlQueryResult, sqlx::Error> {
        if data.id.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }
        let og_song: Song = match self.select_song_by_id(data.id.as_ref().unwrap()).await {
            Ok(res) => match res.is_some() {
                true => res.unwrap(),
                false => return Err(sqlx::Error::RowNotFound),
            },
            Err(_) => return Err(sqlx::Error::RowNotFound),
        };
        sqlx::query!(
            "UPDATE song SET name=?, lyrics=? WHERE id=?",
            data.name
                .unwrap_or(og_song.name.unwrap_or(String::default())),
            data.lyrics
                .unwrap_or(og_song.lyrics.unwrap_or(String::default())),
            data.id,
        )
        .execute(&self.db_pool)
        .await
    }

    pub async fn delete_song(&self, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        sqlx::query!(
            "DELETE FROM song
            WHERE id = ?",
            id
        )
        .execute(&self.db_pool)
        .await
    }
}
