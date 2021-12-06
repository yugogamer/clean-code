use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AuthResponse{
    pub status: String,
    pub request: String,
    pub result: bool
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GenerateResponse{
    pub status: String,
    pub request: String,
    pub result: String
}