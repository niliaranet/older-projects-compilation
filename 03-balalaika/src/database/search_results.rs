use crate::database::album::Album;
use crate::database::artist::Artist;
use crate::database::song::Song;
use crate::database::DatabaseWrapper;

impl DatabaseWrapper {
    pub async fn search_results_by_name(
        &self,
        name: &str,
    ) -> (
        Result<Vec<Artist>, sqlx::Error>,
        Result<Vec<Album>, sqlx::Error>,
        Result<Vec<Song>, sqlx::Error>,
    ) {
        return (
            self.select_artists_by_name(name).await,
            self.select_albums_by_name(name).await,
            self.select_songs_by_name(name).await,
        );
    }
}
