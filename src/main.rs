#[macro_use] extern crate rocket;
extern crate reqwest;

mod service;
mod controlleur;
    

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build();
    let loader = service::database::connection(loader).await;
    loader.launch().await
}