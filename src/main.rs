#[macro_use] extern crate rocket;
extern crate reqwest;

use core::num;

use regex::Regex;

mod service;
mod controlleur;
mod entity;

fn key_finder(id: &str) -> Result<(), anyhow::Error> {
    let mut final_value: u32;
    let mut res: bool = false;
    let id_len: String = id.len().to_string();
    let key_format: Regex = Regex::new(r"(\D{1}[A-Z])(\d{9}[0-9])").unwrap();
    let key_len: String = "10".into();

    if key_format.is_match(id) && id_len == key_len {
        res = true;
    }

    match res {
        false => return Err(anyhow::anyhow!("The ID given is invalid")),
        true => {            
            let id_numbers: Vec<char> = id.chars().collect();
            for numbers in id_numbers.iter() {
                if numbers.is_numeric(){
                    final_value += numbers.to_digit(0-9).unwrap();
                }
            }
            // todo: Conditionning avec le nombre 15 sur la final value (cf: consigne exo)
        }
    };
    Ok(())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let loader = rocket::build();
    let loader = controlleur::authentification::load_road(loader);
    loader.launch().await
}

#[cfg(test)]
mod test{
use super::*;
use test_case::test_case;

#[rustfmt::skip]
#[test_case("","The ID given is invalid".to_string())]
// todo: More test cases ( Real results or errors)
// #[test_case("","".to_string())]
// #[test_case("","".to_string())]
// #[test_case("","".to_string())]
// #[test_case("","".to_string())]

    fn find_the_key(id: &str, expected: String) -> Result<(), anyhow::Error>{
        let the_key = key_finder(id);
        match the_key {
            Err(_) => {
                let err = the_key.expect_err("Should be an error");
                assert_eq!(err.to_string(), expected);
            }
            Ok(_) => {
                assert_eq!(the_key.unwrap_or_default(), expected);
            }
        }
        Ok(())
    }
}
