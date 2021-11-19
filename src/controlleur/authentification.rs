use crate::entity::authentification::AuthResponse;
use crate::service::authentification::generate_id;

pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    return loader.mount("/client/cle/verification", routes![auth_user]);
}


#[get("/<id>")]
async fn auth_user(id: &str) -> String {
    let mut response = AuthResponse {
        status: String::from(""),
        request: String::from(""),
        result: false,
    };

    let key = generate_id(id);

    match key{
        Ok(key) => {
            response.request = String::from(key);
            response.status = String::from("Succes");
            response.result = true;
        },
        Err(err) => {
            response.request = err.to_string();
            response.status = String::from("Errors");
        }
    }

    return serde_json::to_string(&response).unwrap();
}