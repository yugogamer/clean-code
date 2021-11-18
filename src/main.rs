#[macro_use] extern crate rocket;
extern crate reqwest;

mod service;
mod controlleur;
    

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build();
    loader.launch().await
}