use regex::Regex;

const KEY_FORMAT_STRING: &str = r"[A-Z]{1}[0-9]{9}";
const ID_FORMAT_STRING: &str = r"[0-9]{9}";
const KEY_LEN: usize = 10;
const ID_LEN: usize = 9;
const MIN_VALUE: u32 = 100000000;
const MIN_KEY: char = 'Z';

/// A function generating an ID by using a key.
/// 
/// The key given must be valid refering to key_validation() function
/// and generate different ID under specific conditions.
/// 
/// An addition of all numbers in the key has been done
/// if  result > 15, we redo the addition on the result
/// until result < 15.
/// 
/// Then we do final_result+1 and compare it in the
/// alpabet to get the letter equals to the result.
/// 
/// If the letter of the key is the same as the ID final,
/// Result is Ok
/// 
/// # Exceptions
/// If the key number is lower than the const MIN_VALUE (100000000):
/// 
/// Return the ID "Z"
/// 
/// # Examples
/// ```
/// J123456789 -> valid format 1x[A-Z] + 9x[0-9]
/// 1+2+3+4+5+6+7+8+9 = 45 (45 > 15), 4+5 = 9 (9 < 15), 9+1 = 10 : J
/// J = J : Valid result
/// ```
/// 
/// Basic Usages:
/// ```
/// // Valid key format + result
/// let valid_id = verifi_id("J123456789")
/// assert_eq!(verifie_id("F850391564"), "F")
/// 
/// // Valid exception
/// assert_eq!("Z010000000","Z")
/// 
/// // Valid key format but wrong result
/// assert_eq!(verifie_id("A123456789"), "The Key given is invalid")
/// 
/// // Invalid key
/// assert_eq!(verifie_id("j123456789"), "The key format is invalid")
/// ```
pub fn verifie_id(key: &str) -> Result<String, anyhow::Error>{
    let err = key_validation(key);
    let mut key = key;

    match err {
            Ok(_) => {

            }
            Err(e) => return Err(e),
        };
    
    let letter = key.chars().next().unwrap();
    key = &key[1..key.len()];
    let current_key: u32 = key.parse().unwrap();

    match current_key > MIN_VALUE{
        true => {
            if calculate_key(key) == letter {
                return Ok(letter.to_string());
            }
        },
        false => {
            if letter == MIN_KEY{
                return Ok(letter.to_string());
            }
        },
    }
    Err(anyhow::anyhow!("The Key given is invalid"))
}

pub fn generate_id(key: &str) -> Result<char, anyhow::Error>{
    let err = key_gen_validation(key);
    
    match err {
        Ok(_) => {
            let current_key: u32 = key.parse().unwrap();
            if current_key > MIN_VALUE{
                return Ok(calculate_key(key));   
            }else {
                return Ok('Z');
            }
        },
        Err(err) => return Err(err),
    }
}

fn calculate_key(id : &str) -> char{
    let mut sum = 16;
    let mut id = id;
    let mut char_array = Vec::new();
    let mut sum_string : String;

    while sum > 15 {
        for char in id.chars(){
            char_array.push(char);
        }
        
        sum = make_sum(&char_array);
        char_array.clear();
        sum_string = sum.to_string();
        id = sum_string.as_str();
    }

    return ('A' as u8 + sum as u8) as char;
}

fn make_sum(char_array: &Vec<char>) -> u32{
    let mut sum : u32 = 0;
    for number in char_array{
        if number.is_numeric(){
            sum += number.to_digit(10).unwrap();
        }
    }
    sum
}

/// A function using Regex expression to check if the key is valid.
/// 
/// The ID given should respect this format:
/// - 1 capital letter followed by 9 numbers
/// - The letter must be in the range "[A-Z]"
/// - The numbers must be in the range "[0-9]"
/// 
/// Returning an empty String the key is valid
/// 
/// # Examples
/// 
/// Basic usage:
/// ```
/// // Valid formats
/// let valid_key = "A123456789";
/// assert_eq!(valid_key, "");
/// assert_eq!(key_validation("Z000000000"), "");
/// 
/// // Invalid formats
/// assert_eq!(key_validation("a123456789"), "The ID given is invalid"));
/// assert_eq!(key_validation("203950248U"), "The ID given is invalid");
/// assert_eq!(key_validation("4444444444"), "The ID given is invalid");
/// assert_eq!(key_validation("ZEOHZEGPGP"), "The ID given is invalid");
/// assert_eq!(key_validation("2E3950W48U"), "The ID given is invalid");
/// ```
fn key_validation(key: &str) -> Result<(), anyhow::Error>{
    let mut res: bool = false;
    let key_len: usize = key.len();
    let key_format: Regex = Regex::new(KEY_FORMAT_STRING).unwrap();
    
    if key_format.is_match(key) && key_len == KEY_LEN {
        res = true;
    }
    
    match res {
        false => Err(anyhow::anyhow!("The key format is invalid")),
        true => Ok(()),
    }
}

fn key_gen_validation(key: &str) -> Result<(), anyhow::Error>{
    let mut res: bool = false;
    let key_len: usize = key.len();
    let key_format: Regex = Regex::new(ID_FORMAT_STRING).unwrap();
    
    if key_format.is_match(key) && key_len == ID_LEN {
        res = true;
    }
    
    match res {
        false => Err(anyhow::anyhow!("The id format is invalid")),
        true => Ok(()),
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use test_case::test_case;
    
    #[rustfmt::skip]
    #[test_case("12","3")]
    #[test_case("2222222222","20")]
    #[test_case("123456789","45")]
    fn test_make_sum(entry: &str, expected: &str){
        let char_array: Vec<char> = entry.chars().collect();
        let result = make_sum(&char_array);
        assert_eq!(result.to_string().as_str(), expected);
    }
    
    #[test_case("","The key format is invalid")]
    #[test_case("15148A6565","The key format is invalid")]
    #[test_case("a154789623","The key format is invalid")]
    #[test_case("G487845617","")]
    #[test_case("A123456789","")]
    #[test_case("Z010000000","")]
    fn find_the_key(id: &str, expected: &str) -> Result<(), anyhow::Error>{
        let the_key = key_validation(id);
        println!("{}", &id);
        match the_key {
            Err(err) => {
                assert_eq!(err.to_string().as_str(), expected);
            }
            Ok(_) => {
                assert_eq!("", expected);
            }
        }
        Ok(())
    }

    #[test_case("","The id format is invalid")]
    #[test_case("12415148A6565","The id format is invalid")]
    #[test_case("4789623","The id format is invalid")]
    #[test_case("487845617","")]
    #[test_case("123456789","")]
    #[test_case("010000000","")]
    fn test_find_id(id: &str, expected: &str) -> Result<(), anyhow::Error>{
        let the_key = key_gen_validation(id);
        println!("{}", &id);
        match the_key {
            Err(err) => {
                assert_eq!(err.to_string().as_str(), expected);
            }
            Ok(_) => {
                assert_eq!("", expected);
            }
        }
        Ok(())
    }
    
    #[test_case("L985412360","L")]
    #[test_case("Z010000000","Z")]
    #[test_case("A123456789","The Key given is invalid")]
    #[test_case("2222222222","The key format is invalid")]
    fn test_verifie_id(entry: &str, expected: &str){
        let result = verifie_id(entry);
        match result {
            Err(err) => {
                assert_eq!(err.to_string().as_str(), expected)
            },
            Ok(key) => {
                assert_eq!(key, expected)
            }
        }
    }

    #[test_case("985412360","L")]
    #[test_case("010000000","Z")]
    #[test_case("3456789","The id format is invalid")]
    #[test_case("2222222","The id format is invalid")]
    fn test_generate_id(entry : &str, expected: &str){
        let result = generate_id(entry);
        match result {
            Err(err) => {
                assert_eq!(err.to_string().as_str(), expected)
            },
            Ok(key) => {
                assert_eq!(key.to_string(), expected)
            }
        }

    }
}