use crate::entity::authentification::{AuthResponse, GenerateResponse};
use crate::service::authentification::{verifie_id, get_letter};
use rocket::serde::{json::Json};
use rocket_okapi::{openapi, openapi_get_routes};


pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let settings = rocket_okapi::settings::OpenApiSettings::new();
    loader.mount("/client/cle", openapi_get_routes![settings: auth_user, create_id])
}

/// # Key to ID
/// allow to get user ID with a key
#[openapi]
#[get("/verification/<key>", format = "json")]
async fn auth_user(key: &str) -> Json<AuthResponse> {
    let mut response = AuthResponse {
        status: String::from(""),
        request: String::from(key),
        result: false,
    };

    let id = verifie_id(key);

    match id{
        Ok(_) => {
            response.status = String::from("Succes");
            response.result = true;
        },
        Err(_) => {
            response.status = String::from("Errors");
        }
    }

    Json(response)
}

/// # Key to ID
/// allow to get user ID with a key
#[openapi]
#[get("/creation/<key>", format = "json")]
async fn create_id(key: &str) -> Json<GenerateResponse> {
    let mut response = GenerateResponse {
        status: String::from(""),
        request: String::from(""),
        result: "".to_string(),
    };

    let id = get_letter(&key);

    match id{
        Ok(result) => {
            response.request = key.to_string();
            response.status = String::from("Succes");
            response.result = result.to_string();
        },
        Err(err) => {
            response.request = err.to_string();
            response.status = String::from("Errors");
        }
    }

    Json(response)
}