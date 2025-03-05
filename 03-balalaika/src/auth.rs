use crate::database::user::{User, UserForm};
use crate::AppState;
use actix_web::{delete, post, web, HttpResponse, Scope};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
pub use crate::api::Response;
use utoipa::ToSchema;

/* Set up scope */
pub fn auth_scope() -> Scope {
    web::scope("/auth")
        //.service(register)
        .service(login)
        .service(delete_user)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct EncodeResponse {
    #[schema(example = "response")]
    message: String,
    #[schema(example = "4f4bf0b9ef653818a56df74cffb024bd")]
    token: String,
}

#[utoipa::path(
    request_body = UserForm,
    context_path = "/auth",
    description = "Creates a new user with the specified values.",
    responses(
        (status = 200, description = "Create new user", body = Response),
        (status = 400, description = "Errors found, unfulfilled request"),
    ),
)]
#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    request_data: web::Json<UserForm>,
) -> HttpResponse {
    let query = app_state.database.register(request_data.into_inner()).await;
    match query {
        Ok(_) => HttpResponse::Ok().json(Response {
            message: "Registration executed with no errors".to_owned(),
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: format!("There was an issue in the request: {}", e).to_owned(),
        }),
    }
}

#[utoipa::path(
    request_body = UserForm,
    context_path = "/auth",
    description = "Attempts to log in user. If successful, it returns an encoded token that grants access to protected routes in the api.",
    responses(
        (status = 200, description = "Returns encoded token", body = EncodeResponse),
        (status = 400, description = "Errors found, unfulfilled request"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    request_data: web::Json<UserForm>,
) -> HttpResponse {
    let query = app_state.database.login(request_data.into_inner()).await;

    let result = match query {
        Ok(res) => res,
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: format!("There was an issue in the request: {}", e).to_owned(),
            })
        }
    };

    let user: User = match result {
        Some(user) => user,
        None => {
            return HttpResponse::BadRequest().json(Response {
                message: "Username/Password incorrect!".to_owned(),
            })
        }
    };

    let id: usize = match user.id {
        Some(res) => res as usize,
        None => {
            return HttpResponse::BadRequest().json(Response {
                message: "Internal error: user id not found".to_owned(),
            })
        }
    };

    return match encode_token(id, &app_state.secret).await {
        Ok(token) => HttpResponse::Ok().json(EncodeResponse {
            message: format!("Successfully logged in as {}", user.name.unwrap()).to_owned(),
            token: token.to_owned(),
        }),
        Err(response) => response,
    };
}

async fn encode_token(id: usize, secret: &String) -> Result<String, HttpResponse> {
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    ) {
        Ok(token) => return Ok(token),
        Err(_) => return Err(HttpResponse::Ok().body("Token encoding didn't work\n")),
    };
}

#[utoipa::path(
    request_body = UserForm,
    context_path = "/auth",
    description = "Attempts to delete user. Both username and password are required.",
    responses(
        (status = 200, description = "Delete user", body = Response),
        (status = 400, description = "Errors found, unfulfilled request"),
        (status = 401, description = "Unauthorized"),
    ),
)]
#[delete("/user")]
pub async fn delete_user(
    app_state: web::Data<AppState>,
    request_data: web::Json<UserForm>,
) -> HttpResponse {
    let query = app_state
        .database
        .delete_user(request_data.into_inner())
        .await;

    match query {
        Ok(_) => HttpResponse::Ok().json(Response {
            message: "Deletion executed with no errors".to_owned(),
        }),
        Err(e) => {
            return HttpResponse::BadRequest().json(Response {
                message: format!("There was an issue in the request: {}", e).to_owned(),
            })
        }
    }
}
