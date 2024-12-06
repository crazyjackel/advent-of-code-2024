use std::{
    fs::File,
    io::{Read},
};

#[derive(Debug, PartialEq)]
enum Token {
    Do,
    Dont,
    Mul(i32, i32),
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tokens = tokenize(contents);
    
    let mut count = 0;
    let mut do_mult = true;
    for token in tokens{
        match token {
            Token::Mul(left,right) => if do_mult { count += left*right },
            Token::Do => do_mult = true,
            Token::Dont => do_mult = false
        }
    }
    println!("Count is {}", count);
}

fn tokenize(string: String) -> Vec<Token> {
    let chars: Vec<char> = string.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    for i in 0..chars.len() {
        let char = chars.get(i).unwrap();
        let token: Option<Token> = match char {
            'm' => parse_mul_token(&chars, i).ok(),
            'd' => parse_d_token(&chars,i).ok(),
            _ => None,
        };
        if let Some(token) = token {
            tokens.push(token);
        }
    }
    tokens
}

fn parse_d_token(chars: &Vec<char>, i: usize) -> Result<Token, String> {
    // [0,7) "don't()do()" -> "dont()"
    let d_slice_option = chars.get(i..i + 7);
    if !d_slice_option.is_none(){
        let dont_string : String = d_slice_option.unwrap().iter().collect();
        if dont_string == "don't()"{
            return Ok(Token::Dont);
        }
    }

    // [0,4) "do()don't()" -> "do()"
    let d_slice_option2 = chars.get(i..i + 4);
    if !d_slice_option2.is_none(){
        let do_string : String = d_slice_option2.unwrap().iter().collect();
        if do_string == "do()"{
            return Ok(Token::Do);
        }
    }

    return Err("Not a Do or Don't Token".to_string())
}

fn parse_mul_token(chars: &Vec<char>, i: usize) -> Result<Token, String> {
    // [0,4) "mul(66,54)" -> "mul("
    let mul_slice_option = chars.get(i..i + 4);
    if mul_slice_option.is_none(){
        return Err("Mul Token Component is out of bounds".to_string());
    }
    let mul_string : String = mul_slice_option.unwrap().iter().collect();
    if mul_string != "mul("{
        return Err("Not a Mul Token".to_string());
    }

    // [4, len) "mul(66,54)" -> "66,"
    let mut comma_index_option : Option<usize> = None;
    // Since max parseable non-whitespace i32 is 11 chars long, we only need to check 11 additional digits
    for j in i+4..i+16{
        if let Some(char) = chars.get(j) {
            if char.is_whitespace(){
                return Err("Whitespace Found Between Left Paranthesis and Comma Index".to_string());
            }
            if char == &',' { 
                comma_index_option = Some(j);
                break;
            }
        }
    }
    if comma_index_option == None{
        return Err("No Comma Found".to_string());
    }
    let comma_index = comma_index_option.unwrap();

    // [4, 4?16) "mul(66,54)" -> "66"
    let val_one_parse_slice = chars.get(i+4..comma_index);
    if val_one_parse_slice.is_none(){
        return Err("Could not parse left Value Token".to_string());
    }
    let val_one_string : String = val_one_parse_slice.unwrap().iter().collect();
    let val_one_option : Option<i32> = val_one_string.parse().ok();
    if val_one_option.is_none(){
        return Err("Could not parse left Value Token".to_string());
    }
    let val_one = val_one_option.unwrap();

    // [4?16, 4?28) "mul(66,54)" -> "54)"
    let mut right_paranthesis_option : Option<usize> = None;
    // Since max parseable non-whitespace i32 is 11 chars long, we only need to check 11 additional digits
    for j in comma_index..comma_index+12{
        if let Some(char) = chars.get(j) {
            if char.is_whitespace(){
                return Err("Whitespace Found Between Comma Index and Right Paranthesis".to_string());
            }
            if char == &')' { 
                right_paranthesis_option = Some(j);
                break;
            }
        }
    }
    if right_paranthesis_option == None{
        return Err("No Right Paranthesis Found".to_string());
    }
    let right_paranthesis_index = right_paranthesis_option.unwrap();

    // [5?17, 5?29) "mul(66,54)" -> "54"
    let val_two_parse_slice = chars.get(comma_index+1..right_paranthesis_index);
    if val_two_parse_slice.is_none(){
        return Err("Could not parse right Value Token".to_string());
    }

    let val_two_string : String = val_two_parse_slice.unwrap().iter().collect();
    let val_two_option : Option<i32> = val_two_string.parse().ok();
    if val_two_option.is_none(){
        return Err("Could not parse right Value Token".to_string());
    }
    let val_two = val_two_option.unwrap();

    Ok(Token::Mul(val_one, val_two))
}



#[test]
fn parse_valid_dont_token() {
    let valid_chars: Vec<char> = "don't()".chars().collect();
    let result = parse_d_token(&valid_chars, 0);

    assert!(result.is_ok());
    let token = result.unwrap();
    assert_eq!(token, Token::Dont, "{:?} {:?}", token, Token::Dont);
}

#[test]
fn parse_valid_do_token() {
    let valid_chars: Vec<char> = "do()".chars().collect();
    let result = parse_d_token(&valid_chars, 0);

    assert!(result.is_ok());
    let token = result.unwrap();
    assert_eq!(token, Token::Do, "{:?} {:?}", token, Token::Do);
}


#[test]
fn parse_valid_mul_token() {
    let valid_chars: Vec<char> = "mul(44,46)".chars().collect();
    let result = parse_mul_token(&valid_chars, 0);

    assert!(result.is_ok());
    let token = result.unwrap();
    assert_eq!(token, Token::Mul(44, 46), "{:?} {:?}", token, Token::Mul(44, 46));
}

#[test]
fn parse_invalid_whitespace_mul_token() {
    let invalid_chars1: Vec<char> = "mul( 44,46)".chars().collect();
    let result1 = parse_mul_token(&invalid_chars1, 0);
    assert!(result1.is_err());

    let invalid_chars2: Vec<char> = "mul(44, 46)".chars().collect();
    let result2 = parse_mul_token(&invalid_chars2, 0);
    assert!(result2.is_err());
}