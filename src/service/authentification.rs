use regex::Regex;


const KEY_FORMAT_STRING: &str = r"[A-Z]{1}[0-9]{9}";
const KEY_LEN: usize = 10;
const MIN_VALUE: u32 = 10000000;
const MIN_KEY: char = 'Z';




pub fn generate_id(id: &str) -> Result<String, anyhow::Error>{
    let err = key_validation(&id);
    let mut key = id;
    let mut sum_string : String;

    if err.is_err(){
        return Err(err.unwrap_err());
    }

    let mut sum = 16;
    let letter = key.chars().nth(0).unwrap();
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
        
            return Err(anyhow::anyhow!("The Key given is invalid"));
        },
        false => {
            if letter == MIN_KEY{
                return Ok(letter.to_string());
            }
            return Err(anyhow::anyhow!("The Key given is invalid"));
        },
    }
}


fn make_sum(char_array: &Vec<char>) -> u32{
    let mut sum : u32 = 0;
    for number in char_array{
        if number.is_numeric(){
            sum += number.to_digit(10).unwrap();
        }
    }
    return sum;
}

fn key_validation(id: &str) -> Result<(), anyhow::Error>{
    let mut res: bool = false;
    let id_len: usize = id.len();
    let key_format: Regex = Regex::new(KEY_FORMAT_STRING).unwrap();
    
    
    if key_format.is_match(id) && id_len == KEY_LEN {
        res = true;
    }
    
    match res {
        false => return Err(anyhow::anyhow!("The ID given is invalid")),
        true => return Ok(()),
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
        let result = make_sum(&char_array);
        assert_eq!(result.to_string(), expected);
    }
    
    
    #[rustfmt::skip]
    #[test_case("","The ID given is invalid".to_string())]
    #[test_case("15148A6565","The ID given is invalid".to_string())]
    #[test_case("a154789623","The ID given is invalid".to_string())]
    #[test_case("Z010000000","".to_string())]
    #[test_case("B897561789","".to_string())]
    #[test_case("F487845617","".to_string())]
    #[test_case("A123456789","".to_string())]
    
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
    #[test_case("Z010000000","Z".to_string())]
    #[test_case("2222222222","The ID given is invalid".to_string())]
    #[test_case("L985412360","L".to_string())]
    fn test_generate_id(entry: &str, expected: String){
        let result = generate_id(&entry);
        match result {
            Err(err) => {
                assert_eq!(err.to_string(), expected)
            },
            Ok(key) => {
                assert_eq!(key.to_string(), expected)
            }
        }
    }
}