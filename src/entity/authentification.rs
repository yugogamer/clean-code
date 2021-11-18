use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse{
    pub status: String,
    pub request: String,
    pub result: bool
}