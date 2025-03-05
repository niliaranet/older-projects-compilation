use crate::auth::Claims;
use crate::AppState;
use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http, http::header::HeaderValue, web,
    Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationToken {
    pub id: usize,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // get auth token from the authorization header
        let auth_header: &HeaderValue = match req.headers().get(http::header::AUTHORIZATION) {
            Some(res) => res,
            None => return ready(Err(ErrorUnauthorized("No authorization token given"))),
        };

        /* Get value as &str */
        let processed_header: &str = auth_header.to_str().unwrap_or("");
        if processed_header.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid auth token!")));
        }

        /* Accept both bearer tokens and raw tokens */
        let splitted_header: Vec<&str> = processed_header.split_whitespace().collect();
        let auth_token: String = match splitted_header[0] {
            "Bearer" => splitted_header[1].to_string(),
            _ => processed_header.to_string(),
        };

        /* Get application secret */
        let secret: String = req
            .app_data::<web::Data<AppState>>()
            .unwrap()
            .secret
            .to_string();

        /* Decode token with secret */
        let decode: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        // for testing purposes
        println!("{}", auth_token);

        /* Return authenticationtoken */
        match decode {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.id,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Unauthorized!"))),
        }
    }
}

/* Example execution in curl:
curl localhost:8000/auth/protected -H "Accept: application/json" -H "Authorization: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwiZXhwIjoxNzY5MjAyNjU1fQ.QbWkgjmbmMwLJnia6vd67EfRkf6y-4nw572g-Nk0BOE"
*/
