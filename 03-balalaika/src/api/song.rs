use crate::api::{get_response_from_query, Response};
use crate::database::song::{Song, SongPost, SongPut};
use crate::database::Delete;
use crate::extractors::auth_token::AuthenticationToken;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use utoipa::IntoParams;

/* Possible arguments ( /song?arg=value ) */
#[derive(Deserialize, IntoParams)]
struct SongQueryOptions {
    id: Option<String>,
    name: Option<String>,
    artist: Option<String>,
    album: Option<String>,
}

#[utoipa::path(
    params(SongQueryOptions),
    context_path = "/api",
    description = "Gets a list of the current songs and applies filters based on the url parameters recieved. It only accepts one parameter at a time.",
    responses(
        (status = 200, description = "Return a list of songs", body = Vec<Song>),
        (status = 400, description = "Errors found, unfulfilled request"),
    ),
)]
#[get("/song")]
pub async fn get_song(
    app_state: web::Data<AppState>,
    get_args: web::Query<SongQueryOptions>,
) -> HttpResponse {
    /* Avoid lifespan issues */
    let default = String::default();

    /* Handle individual result for search-by-id */
    if get_args.id.is_some() {
        let id: &str = get_args.id.as_ref().unwrap_or(&default);
        let query_result: sqlx::Result<Option<Song>> =
            app_state.database.select_song_by_id(id).await;

        return match query_result {
            Ok(song_list) => HttpResponse::Ok().json(song_list),
            Err(e) => HttpResponse::Ok().body(format!("{}", e)),
        };
    }

    /* Handle N results */
    let query_result: sqlx::Result<Vec<Song>> = match true {
        _ if get_args.name.is_some() => {
            let name: &str = get_args.name.as_ref().unwrap_or(&default);
            app_state.database.select_songs_by_name(name).await
        }
        _ if get_args.album.is_some() => {
            let album: &str = get_args.album.as_ref().unwrap_or(&default);
            app_state.database.select_songs_by_album(album).await
        }
        _ if get_args.artist.is_some() => {
            let artist: &str = get_args.artist.as_ref().unwrap_or(&default);
            app_state.database.select_songs_by_artist(artist).await
        }
        _ => app_state.database.select_songs().await,
    };

    match query_result {
        Ok(song_list) => HttpResponse::Ok().json(song_list),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

#[utoipa::path(
    request_body = SongPost,
    context_path = "/api",
    description = "Creates a new song with the specified values.",
    responses(
        (status = 200, description = "Create new song", body = Response),
        (status = 400, description = "Errors found, unfulfilled request"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[post("/song")]
pub async fn post_song(
    app_state: web::Data<AppState>,
    request_data: web::Json<SongPost>,
    _auth_token: AuthenticationToken,
) -> HttpResponse {
    get_response_from_query(
        app_state
            .database
            .create_song(request_data.into_inner())
            .await,
        "POST".to_string(),
    )
}

#[utoipa::path(
    request_body = SongPut,
    context_path = "/api",
    description = "Edits the values of the specified song.",
    responses(
        (status = 200, description = "Edit song values", body = Response),
        (status = 400, description = "Errors found or song not found"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[put("/song")]
pub async fn put_song(
    app_state: web::Data<AppState>,
    request_data: web::Json<SongPut>,
    _auth_token: AuthenticationToken,
) -> HttpResponse {
    get_response_from_query(
        app_state
            .database
            .edit_song(request_data.into_inner())
            .await,
        "PUT".to_owned(),
    )
}

#[utoipa::path(
    request_body = Delete,
    context_path = "/api",
    description = "Deletes the specified song.",
    responses(
        (status = 200, description = "Delete existing song", body = Response),
        (status = 400, description = "Errors found or song not found"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[delete("/song")]
pub async fn delete_song(
    app_state: web::Data<AppState>,
    request_data: web::Json<Delete>,
    _auth_token: AuthenticationToken,
) -> HttpResponse {
    /* Check if ID is valid (return -1 if invalid) */
    let id: i32 = request_data
        .into_inner()
        .id
        .unwrap_or(String::default())
        .parse::<i32>()
        .unwrap_or(-1);

    if id == -1 {
        return HttpResponse::BadRequest().json(Response {
            message: "Invalid id value, code not executed".to_owned(),
        });
    }

    get_response_from_query(
        app_state.database.delete_song(id).await,
        "DELETE".to_owned(),
    )
}
