#[macro_use] extern crate rocket;
extern crate reqwest;

mod service;
mod controlleur;
mod entity;
    

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build();
    let loader = controlleur::authentification::load_road(loader);
    loader.launch().await
}