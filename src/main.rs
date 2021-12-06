#[macro_use] extern crate rocket;
use rocket_okapi::{swagger_ui::*};

mod service;
mod controlleur;
mod entity;



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build();
    let loader = controlleur::authentification::load_road(loader);
    let loader = loader.mount(
        "/doc/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "/client/cle/openapi.json".to_owned(),
            ..Default::default()
        }),
    );
    loader.launch().await
}