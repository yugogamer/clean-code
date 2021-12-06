use crate::entity::authentification::AuthResponse;
use crate::service::authentification::generate_id;
use rocket::serde::{json::Json};
use rocket_okapi::{openapi, openapi_get_routes};


pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    loader.mount("/client/cle/verification", openapi_get_routes![settings: auth_user])
}

/// # Key to ID
/// allow to get user ID with a key
#[openapi]
#[get("/<key>", format = "json")]
async fn auth_user(key: &str) -> Json<AuthResponse> {
    let mut response = AuthResponse {
        status: String::from(""),
        request: String::from(""),
        result: false,
    };

    let id = generate_id(key);

    match id{
        Ok(key) => {
            response.request = key;
            response.status = String::from("Succes");
            response.result = true;
        },
        Err(err) => {
            response.request = err.to_string();
            response.status = String::from("Errors");
        }
    }

    Json(response)
}