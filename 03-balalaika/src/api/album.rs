use crate::api::{get_response_from_query, Response};
use crate::database::album::{Album, AlbumPost, AlbumPut};
use crate::database::Delete;
use crate::extractors::auth_token::AuthenticationToken;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use utoipa::IntoParams;

/* Possible arguments ( /album?arg=value ) */
#[derive(Deserialize, IntoParams)]
struct AlbumQueryOptions {
    id: Option<String>,
    name: Option<String>,
    artist: Option<String>,
}

#[utoipa::path(
    params(AlbumQueryOptions),
    context_path = "/api",
    description = "Gets a list of the current albums and applies filters based on the url parameters recieved. It only accepts one parameter at a time.",
    responses(
        (status = 200, description = "Return a list of albums", body = Vec<Album>),
        (status = 400, description = "Errors found, unfulfilled request"),
    ),
)]
#[get("/album")]
pub async fn get_album(
    app_state: web::Data<AppState>,
    get_args: web::Query<AlbumQueryOptions>,
) -> HttpResponse {
    /* Avoid lifespan issues */
    let default = String::default();

    /* Handle individual result for search-by-id */
    if get_args.id.is_some() {
        let id: &str = get_args.id.as_ref().unwrap_or(&default);
        let query_result: sqlx::Result<Option<Album>> =
            app_state.database.select_album_by_id(id).await;

        return match query_result {
            Ok(song_list) => HttpResponse::Ok().json(song_list),
            Err(e) => HttpResponse::Ok().body(format!("{}", e)),
        };
    }

    /* Handle N results */
    let query_result: sqlx::Result<Vec<Album>, sqlx::Error> = match true {
        _ if get_args.name.is_some() => {
            let name: &str = &get_args.name.as_ref().unwrap_or(&default);
            app_state.database.select_albums_by_name(name).await
        }
        _ if get_args.artist.is_some() => {
            let artist: &str = &get_args.artist.as_ref().unwrap_or(&default);
            app_state.database.select_albums_by_artist(artist).await
        }
        _ => app_state.database.select_albums().await,
    };

    match query_result {
        Ok(album_list) => HttpResponse::Ok().json(album_list),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

#[utoipa::path(
    request_body = AlbumPost,
    context_path = "/api",
    description = "Creates a new album with the specified values.",
    responses(
        (status = 200, description = "Create new album", body = Response),
        (status = 400, description = "Errors found, unfulfilled request"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[post("/album")]
pub async fn post_album(
    app_state: web::Data<AppState>,
    request_data: web::Json<AlbumPost>,
    _auth_token: AuthenticationToken,
) -> HttpResponse {
    get_response_from_query(
        app_state
            .database
            .create_album(request_data.into_inner())
            .await,
        "POST".to_string(),
    )
}

#[utoipa::path(
    request_body = AlbumPut,
    context_path = "/api",
    description = "Edits the values of the specified album.",
    responses(
        (status = 200, description = "Edit album values", body = Response),
        (status = 400, description = "Errors found or album not found"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[put("/album")]
pub async fn put_album(
    app_state: web::Data<AppState>,
    request_data: web::Json<AlbumPut>,
    _auth_token: AuthenticationToken,
) -> HttpResponse {
    get_response_from_query(
        app_state
            .database
            .edit_album(request_data.into_inner())
            .await,
        "PUT".to_string(),
    )
}

#[utoipa::path(
    request_body = Delete,
    context_path = "/api",
    description = "Deletes the specified album.",
    responses(
        (status = 200, description = "Delete existing album", body = Response),
        (status = 400, description = "Errors found or album not found"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[delete("/album")]
pub async fn delete_album(
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
            message: "Invalid id value, code not executed\n".to_owned(),
        });
    }

    get_response_from_query(
        app_state.database.delete_album(id).await,
        "POST".to_string(),
    )
}
