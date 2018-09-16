use std::str;
use std::fmt;
use std::char;
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
pub fn from_decimal(input:String)->String{
    let mut result : String = String::new();
    let number_string = to_big_number(input);
    let number = number_string.parse::<i64>();
    if number.is_ok(){
        let temp = number.unwrap();
        if(temp<0){
            result.insert_str(0,"-0x");
            let length = result.len();
            let temp = 0-temp;
            let temp_string = format!("{:x}", temp);
            result.insert_str(length,temp_string.as_str());
        }else{
            result.insert_str(0,"0x");
            let length = result.len();
            let temp_string = format!("{:x}", temp);
            result.insert_str(length,temp_string.as_str());
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
    let regex = Regex::new(r"(?i)(^0x[0-9a-f]{64}$)").unwrap();
    let result = regex.is_match(address.as_str());
    result
}
pub fn is_checksum_address(address : String)->bool{
    let mut result:bool = true;
    let mut address_clone:String = address.clone();
    if address_clone.len() >= 2 {
        if &address_clone[0..2] == "0x" {
            address_clone = (&address[2..address.len()]).to_string();
        }
    }
    let hash = sha3::hash(address_clone.to_lowercase());
    let mut i = 0 ;
    while i<=hash.len()-1 {
        let temp = i64::from_str_radix(&hash[i..i+1], 16);
        if temp.is_ok(){
            let temp = temp.unwrap();
            if (temp > 7 &&  address_clone[i..i+1].to_uppercase() != address_clone[i..i+1])||(temp <= 7 &&  address_clone[i..i+1].to_lowercase() != address_clone[i..i+1]){
                result = false;
            }
        }
        i += 1;
    };
    result
}
pub fn to_checksum_address(address:String)->String{
    let mut result:String = "0x".to_string();
    let mut address:String = address.to_lowercase();
    if address.len() >= 2 {
        if &address[0..2] == "0x" {
            address = (&address[2..address.len()]).to_string();
        }
    }
    let hash : String = sha3::hash(address.clone());
    let mut i = 0 ;
    let address_clone = address.clone();
    while i<=hash.len()-1 {
        let temp = i64::from_str_radix(&hash[i..i+1], 16);
        if temp.is_ok(){
            let temp = temp.unwrap();
            let index =result.len();
            if temp>7{
                result.insert_str(index,address_clone[i..i+1].to_uppercase().as_str());
            }else{
                result.insert_str(index,&address_clone[i..i+1]);
            }
        }
        i += 1;
    };
    result
}

pub fn is_address(address : String)->bool{
    let regex_one = Regex::new(r"(?i)(!^(0x)?[0-9a-f]{64}$)").unwrap();
    let regex_two = Regex::new(r"^(0x)?[0-9a-f]{64}$").unwrap();
    let regex_three = Regex::new(r"^(0x)?[0-9A-F]{64}$").unwrap();
    if regex_one.is_match(address.as_str()){
        false
    } else if regex_two.is_match(address.as_str()) || regex_three.is_match(address.as_str()){
        true
    }else{
        is_checksum_address(address)
    }
}

pub trait ToHex<T> {
    fn to_hex(input:T)->String;
}
impl ToHex<bool> for bool {
     fn to_hex(input:bool)->String{
        if input{
            "0x1".to_string()
        }else{
            "0x0".to_string()
        }
    }
}
impl ToHex<String> for String {
     fn to_hex(input:String)->String{
         let mut result : String = String::new();

         if input.len() > 2{
             if &input[0..3] == "-0x" {
                result = from_decimal(input);
            }else if &input[0..2] == "0x"{
                result = input;
            }else{
                result = from_ascii(input);
            }
         }
         result
    }
}

pub fn to_address(input: String)->String{
    let mut result : String = String::new();
    let regex = Regex::new(r"^[0-9a-f]{64}$").unwrap();
    if is_strict_address(input.clone()) {
        result = input.clone();
    } else if regex.is_match(input.as_str()){
        result = input.clone();
        result.insert_str(0,"0x");
    }else{
        result = "0x".to_string();
        let hex_input = String::to_hex(input.clone());
        let hex_input = (&hex_input[2..hex_input.len()]).to_string();
        let result_padded = pad_left(hex_input,64,"0".to_string());
        result.insert_str(2,result_padded.as_str());
    }
    result
}
