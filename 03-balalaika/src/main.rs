mod api;
mod auth;
mod database;
mod doc;
mod extractors;

use actix_web::web::Redirect;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
struct AppState {
    database: Arc<database::DatabaseWrapper>,
    secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Enable the log */
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    /* Create database wrapper (reference: acsim) */
    let db_raw = match database::DatabaseWrapper::new().await {
        Ok(res) => res,
        Err(_) => panic!("Error creating database wrapper"),
    };
    let db = Arc::new(db_raw);

    /* Get jwt secret from env */
    let jwt_secret: String = env!("SECRET").to_owned();

    /* Application data struct */
    let app_state = AppState {
        database: db,
        secret: jwt_secret,
    };

    /* OpenApi */
    let openapi = doc::get_openapi();

    /* Server setup */
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(api::api_scope())
            .service(auth::auth_scope())
            .service(SwaggerUi::new("/doc/{_:.*}").url("/docs/openapi.json", openapi.clone()))
            .route("/", web::get().to(redirect_to_docs))
            .route("/doc", web::get().to(redirect_to_docs))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

/* Redirection page */
async fn redirect_to_docs() -> impl Responder {
    Redirect::to("/doc/")
}
