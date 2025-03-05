use crate::api::{get_response_from_query, Response};
use crate::database::artist::{Artist, ArtistPost, ArtistPut};
use crate::database::Delete;
use crate::extractors::auth_token::AuthenticationToken;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use utoipa::IntoParams;

/* Possible arguments ( /artist?arg=value ) */
#[derive(Deserialize, IntoParams)]
pub struct ArtistQueryOptions {
    id: Option<String>,
    name: Option<String>,
}

#[utoipa::path(
    params(ArtistQueryOptions),
    context_path = "/api",
    description = "Gets a list of the current artists and applies filters based on the url parameters recieved. It only accepts one parameter at a time.",
    responses(
        (status = 200, description = "Return a list of artists", body = Vec<Artist>),
        (status = 400, description = "Errors found, unfulfilled request"),
    ),
)]
#[get("/artist")]
pub async fn get_artist(
    app_state: web::Data<AppState>,
    get_args: web::Query<ArtistQueryOptions>,
) -> HttpResponse {
    /* Avoid lifespan issues */
    let default = String::default();

    /* Handle individual result for search-by-id */
    if get_args.id.is_some() {
        let id: &str = get_args.id.as_ref().unwrap_or(&default);
        let query_result: sqlx::Result<Option<Artist>> =
            app_state.database.select_artist_by_id(id).await;

        return match query_result {
            Ok(song_list) => HttpResponse::Ok().json(song_list),
            Err(e) => HttpResponse::Ok().body(format!("{}", e)),
        };
    }

    /* Handle N results */
    let query_result: sqlx::Result<Vec<Artist>, sqlx::Error> = match true {
        _ if get_args.name.is_some() => {
            let name: &str = &get_args.name.as_ref().unwrap_or(&default);
            app_state.database.select_artists_by_name(name).await
        }
        _ => app_state.database.select_artists().await,
    };

    match query_result {
        Ok(artist_list) => HttpResponse::Ok().json(artist_list),
        Err(e) => HttpResponse::Ok().body(format!("{}", e)),
    }
}

#[utoipa::path(
    request_body = ArtistPost,
    context_path = "/api",
    description = "Creates a new artist with the specified name.",
    responses(
        (status = 200, description = "Create new artist", body = Response),
        (status = 400, description = "Errors found, unfulfilled request"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[post("/artist")]
pub async fn post_artist(
    app_state: web::Data<AppState>,
    request_data: web::Json<ArtistPost>,
    _auth_token: AuthenticationToken,
) -> HttpResponse {
    get_response_from_query(
        app_state
            .database
            .create_artist(request_data.into_inner())
            .await,
        "POST".to_string(),
    )
}

#[utoipa::path(
    request_body = ArtistPut,
    context_path = "/api",
    description = "Edits the name of the specified artist.",
    responses(
        (status = 200, description = "Edit artist name", body = Response),
        (status = 400, description = "Errors found or artist not found"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[put("/artist")]
pub async fn put_artist(
    app_state: web::Data<AppState>,
    request_data: web::Json<ArtistPut>,
    _auth_token: AuthenticationToken,
) -> HttpResponse {
    get_response_from_query(
        app_state
            .database
            .edit_artist(request_data.into_inner())
            .await,
        "PUT".to_string(),
    )
}

#[utoipa::path(
    request_body = Delete,
    context_path = "/api",
    description = "Deletes the specified artist.",
    responses(
        (status = 200, description = "Delete existing artist", body = Response),
        (status = 400, description = "Errors found or artist not found"),
        (status = 401, description = "Authentication failed"),
    ),
    security(
        {"bearer_auth" = []}
    ),
)]
#[delete("/artist")]
pub async fn delete_artist(
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
        app_state.database.delete_artist(id).await,
        "DELETE".to_string(),
    )
}
