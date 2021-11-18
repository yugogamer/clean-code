







fn make_sum(char_array: Vec<char>) -> u32{
    let mut sum : u32 = 0;
    for number in char_array{
        if number.is_numeric(){
            sum += number.to_digit(10).unwrap();
        }
    }
    return sum;
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
        let result = make_sum(char_array);
        assert_eq!(result.to_string(), expected);
    }
}