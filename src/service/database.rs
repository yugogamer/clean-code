use tokio_postgres::{NoTls, Error};
use std::future::Future;
use rocket::State;

pub const DB_HOST : &str = "localhost";
pub const DB_LOGIN : &str= "postgres";
pub const DB_PASSWORD : &str = "postgres";
pub const DB : &str = "anime";

pub struct Database {
    client: tokio_postgres::Client
}



pub async fn connection(loader : rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {

	let mut config =  tokio_postgres::config::Config::new();
	config.user(&DB_LOGIN);
	config.password(&DB_PASSWORD);
	config.host(&DB_HOST);

	let future =  config.connect(NoTls);

	let response = future.await;

	match &response{
		Ok(v) => println!("Datbase connection start"),
		Err(e) => {println!("{}", e) ; return loader},
	}

	let (client, connection) = response.unwrap();
	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("connection error: {}", e);
		}else{
			println!("database connection established");
		}
	});
	return loader.manage(Database{client : client});
}