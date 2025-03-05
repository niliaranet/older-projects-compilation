use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod album;
pub mod artist;
pub mod search_results;
pub mod song;

/* Set up scope */
pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(song::get_song)
        .service(song::post_song)
        .service(song::put_song)
        .service(song::delete_song)
        .service(album::get_album)
        .service(album::post_album)
        .service(album::put_album)
        .service(album::delete_album)
        .service(artist::get_artist)
        .service(artist::post_artist)
        .service(artist::put_artist)
        .service(artist::delete_artist)
        .service(search_results::search_results)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
    #[schema(default = "response")]
    pub message: String,
}

pub fn get_response_from_query(
    query: Result<sqlx::mysql::MySqlQueryResult, sqlx::Error>,
    method: String,
) -> HttpResponse {
    match query {
        Ok(_) => HttpResponse::Ok().json(Response {
            message: format!("{} request executed with no errors", method).to_owned(),
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: format!("There was an issue in the request: {}", e).to_owned(),
        }),
    }
}
