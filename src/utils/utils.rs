use std::str;
use hex::decode;

pub fn pad_left(input : String,shift : usize , sign_bit : String) -> String{
    let mut result = input.clone();
    let mut padding : &str = &sign_bit;
    if sign_bit == String::new(){
        padding = "0";
    }
    for _ in 0..shift{
        result.insert_str(0,padding);
    }
    result
}

pub fn pad_right(input : String,shift : usize , sign_bit : String) -> String{
    let mut result = input.clone();
    let mut padding : &str = &sign_bit;
    if sign_bit == String::new(){
        padding = "0";
    }
    for _ in 0..shift{
        result.insert_str(input.len(),padding);
    }
    result
}

// hex::decode("48656c6c6f20776f726c6421"),
//     Ok("Hello world!".to_owned().into_bytes())

pub fn to_utf8(input : String) -> String{

    let mut result : String = String::new();
    let mut input_clone = input.clone();
    if input.len() >= 2 {          
            if &input[0..2] == "0x" {
                input_clone = (&input[2..input.len()]).to_string();      
            }
            if input_clone.len()%2 == 1 {
                input_clone = (&input_clone[0..input_clone.len()-1]).to_string();  
            }
                let result_temp = decode(input_clone.as_str());
                let result_temp = result_temp.ok();
                let result_temp = result_temp.unwrap_or(Vec::new());
                let result_temp = str::from_utf8(&result_temp).unwrap();
                result = result_temp.to_string();
    };
    result
}


pub fn to_ascii(input : String) -> String{
    let mut result : String = String::new();
    let mut input_clone = input.clone();
    if input.len() >= 2 {          
            if &input[0..2] == "0x" {
                input_clone = (&input[2..input.len()]).to_string();      
            }
            if input_clone.len()%2 == 1 {
                input_clone = (&input_clone[0..input_clone.len()-1]).to_string();  
            }
            let mut i = 0 ; 
            while i<=input_clone.len()-2 {
                let temp = u8::from_str_radix(&input_clone[i..i+2], 16).map(|n| n as char);
                if temp.is_ok(){
                    result.push(temp.ok().unwrap());
                }
                i += 2;
            };
    };
    result
}