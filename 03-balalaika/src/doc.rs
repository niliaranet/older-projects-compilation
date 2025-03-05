use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use crate::auth;

use crate::api;

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    /* utoipa setup */
    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::album::post_album,
            api::album::put_album,
            api::album::get_album,
            api::album::delete_album,
            api::artist::post_artist,
            api::artist::put_artist,
            api::artist::get_artist,
            api::artist::delete_artist,
            api::song::post_song,
            api::song::put_song,
            api::song::get_song,
            api::song::delete_song,
            api::search_results::search_results,
            auth::register,
            auth::login,
            auth::delete_user,
        ),
        components(
            schemas(
            )
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }

    return ApiDoc::openapi();
}
