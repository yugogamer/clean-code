use crate::entity::authentification::AuthResponse;

pub fn load_road(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    return loader.mount("/client/cle/verification", routes![authUser]);
}


#[get("/<id>")]
async fn authUser(id: &str) -> String {
    let mut response = AuthResponse {
        status: String::from(""),
        request: String::from(""),
        result: false,
    };

    

    return serde_json::to_string(&response).unwrap();
}