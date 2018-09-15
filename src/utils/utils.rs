use std::str;
use hex::decode;
use hex::encode;
use regex::Regex;
use super::sha3;

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
pub fn from_utf8(input : String) -> String{
    let input_clone = input.clone();
    let mut result_temp = encode(input_clone.as_str());
    result_temp.insert_str(0,"0x");
    let result : String = result_temp.to_string();
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
pub fn from_ascii(input : String) -> String{
    let mut result = encode(input);
    result.insert_str(0,"0x");
    result
}
pub fn to_decimal(input: String)->String{
    let mut result :String = String::new();
    let mut input_clone = input.clone();
    if input.len() >= 2 {
        if &input[0..2] == "0x" {
            input_clone = (&input[2..input.len()]).to_string();
        }
        let temp = i64::from_str_radix(input_clone.as_str(), 16);
        if temp.is_ok(){
            result = temp.unwrap().to_string();
        }
    }
    result
}
pub fn to_big_number(input:String) -> String
{
    let mut result :String = String::new();
    let mut input_clone = input.clone();
    let mut hex : bool = false;
    if input.len() >= 2 {
        if &input[0..2] == "0x" {
            input_clone = (&input[2..input.len()]).to_string();
            hex = true;
        }
        if input.len() > 2{
            if &input[0..3] == "-0x" {
                input_clone = (&input[3..input.len()]).to_string();
                input_clone.insert_str(0,"-");
                hex = true;
            }
        }
    }
    if hex{
        let temp = i64::from_str_radix(input_clone.as_str(), 16);
        if temp.is_ok(){
            result = temp.unwrap().to_string();
        }
    }else{
        let temp = i64::from_str_radix(input_clone.as_str(), 10);
        if temp.is_ok(){
            result = temp.unwrap().to_string();
        }
    }

    result
    
}
pub fn is_strict_address(address : String)->bool{
    let regex = Regex::new(r"^0x[0-9a-f]{64}$").unwrap();
    let result = regex.is_match(address.as_str());
    result
}
pub fn is_checksum_address(address : String)->bool{
    let mut result:bool = true;
    let hash = sha3::hash(address.to_lowercase());
    println!("{:?}",hash);
    let mut i = 0 ; 
    while i<=hash.len()-1 {
        let temp = i64::from_str_radix(&hash[i..i+1], 16);
        if temp.is_ok(){
            let temp = temp.unwrap();
            if (temp > 7 &&  hash[i..i+1].to_uppercase() != hash[i..i+1])||(temp <= 7 &&  hash[i..i+1].to_lowercase() != hash[i..i+1]){
                result = false;
            }
            // println!("{:?}",temp);
        }
        i += 1;
    };
    result
}
//   if ((parseInt(addressHash[i], 16) > 7 && address[i].toUpperCase() !== address[i]) || (parseInt(addressHash[i], 16) <= 7 && address[i].toLowerCase() !== address[i])) {
//             return false;
//         }