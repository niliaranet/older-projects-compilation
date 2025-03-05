use crate::database::album::Album;
use crate::database::artist::Artist;
use crate::database::song::Song;
use crate::AppState;
use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
struct SearchQueryOptions {
    name: Option<String>,
}

#[utoipa::path(
    params(SearchQueryOptions),
    context_path = "/api",
    description = "Performs a search based on the 'name' parameter and returns a list.",
    responses(
        (status = 200, description = "Return a list of artists, albums and songs"),
        (status = 400, description = "Errors found, unfulfilled request"),
    ),
)]
#[get("/search-results")]
pub async fn search_results(
    app_state: web::Data<AppState>,
    get_args: web::Query<SearchQueryOptions>,
) -> HttpResponse {
    let search_attempt: (
        sqlx::Result<Vec<Artist>, sqlx::Error>,
        sqlx::Result<Vec<Album>, sqlx::Error>,
        sqlx::Result<Vec<Song>, sqlx::Error>,
    ) = match true {
        _ if get_args.name.is_some() => {
            app_state
                .database
                .search_results_by_name(&get_args.name.clone().unwrap())
                .await
        }
        _ => (
            Err(sqlx::Error::RowNotFound),
            Err(sqlx::Error::RowNotFound),
            Err(sqlx::Error::RowNotFound),
        ),
    };

    return HttpResponse::Ok().json((
        search_attempt.0.unwrap_or(Vec::new()),
        search_attempt.1.unwrap_or(Vec::new()),
        search_attempt.2.unwrap_or(Vec::new()),
    ));
}
