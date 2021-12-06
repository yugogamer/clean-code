use regex::Regex;

const KEY_FORMAT_STRING: &str = r"[A-Z]{1}[0-9]{9}";
const KEY_LEN: usize = 10;
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
/// let valid_id = generate_id("J123456789")
/// assert_eq!(generate_id("F850391564"), "F".to_string())
/// 
/// // Valid exception
/// assert_eq!("Z010000000","Z".to_string())
/// 
/// // Valid key format but wrong result
/// assert_eq!(generate_id("A123456789"), "The Key given is invalid".to_String())
/// 
/// // Invalid key
/// assert_eq!(generate_id("j123456789"), "The key format is invalid".to_string())
/// ```
pub fn generate_id(key: &str) -> Result<String, anyhow::Error>{
    let err = key_validation(key);
    let mut key = key;
    let mut sum_string : String;

    match err {
            Ok(_) => {

            }
            Err(e) => return Err(e),
        };
    
    let mut sum = 16;
    let letter = key.chars().next().unwrap();
    key = &key[1..key.len()];
    let mut char_array = Vec::new();
    let current_key: u32 = key.parse().unwrap();

    match current_key > MIN_VALUE{
        true => {
            while sum > 15 {
                for char in key.chars(){
                    char_array.push(char);
                }
                
                sum = make_sum(&char_array);
                char_array.clear();
                sum_string = sum.to_string();
                key = sum_string.as_str();
            }
        
            if ('A' as u32 +sum) == letter as u32 {
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


fn make_sum(char_array: &[char]) -> u32{
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
/// assert_eq!(valid_key, "".to_string());
/// assert_eq!(key_validation("Z000000000"), "".to_string());
/// 
/// // Invalid formats
/// assert_eq!(key_validation("a123456789"), "The ID given is invalid".to_string());
/// assert_eq!(key_validation("203950248U"), "The ID given is invalid".to_string());
/// assert_eq!(key_validation("4444444444"), "The ID given is invalid".to_string());
/// assert_eq!(key_validation("ZEOHZEGPGP"), "The ID given is invalid".to_string());
/// assert_eq!(key_validation("2E3950W48U"), "The ID given is invalid".to_string());
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

#[cfg(test)]
mod test{
    use super::*;
    use test_case::test_case;
    
    #[rustfmt::skip]
    #[test_case("12","3".to_string())]
    #[test_case("2222222222","20".to_string())]
    #[test_case("123456789","45".to_string())]
    fn test_make_sum(entry: &str, expected: String){
        let char_array = entry.chars().collect();
        let result = make_sum(entry);
        assert_eq!(result.to_string(), expected);
    }
    
    #[rustfmt::skip]
    #[test_case("","The key format is invalid".to_string())]
    #[test_case("15148A6565","The key format is invalid".to_string())]
    #[test_case("a154789623","The key format is invalid".to_string())]
    #[test_case("G487845617","".to_string())]
    #[test_case("A123456789","".to_string())]
    #[test_case("Z010000000","".to_string())]

    fn find_the_key(id: &str, expected: String) -> Result<(), anyhow::Error>{
        let the_key = key_validation(id);
        println!("{}", &id);
        match the_key {
            Err(err) => {
                assert_eq!(err.to_string(), expected);
            }
            Ok(_) => {
                assert_eq!("", expected);
            }
        }
        Ok(())
    }

    #[rustfmt::skip]
    
    #[test_case("L985412360","L".to_string())]
    #[test_case("Z010000000","Z".to_string())]
    #[test_case("A123456789","The Key given is invalid".to_string())]
    #[test_case("2222222222","The key format is invalid".to_string())]
    
    fn test_generate_id(entry: &str, expected: String){
        let result = generate_id(entry);
        match result {
            Err(err) => {
                assert_eq!(err.to_string(), expected)
            },
            Ok(key) => {
                assert_eq!(key, expected)
            }
        }
    }
}