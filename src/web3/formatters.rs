use super::super::utils::utils;
use super::super::utils::utils::ToHex;
use super::super::utils::config;

pub fn input_address_formatter(address : String) -> String{
    if utils::is_strict_address(address.clone()) {
        address
    } else if utils::is_address(address.clone()) {
        let mut address = address.clone();
        if address.len() >= 2 {
            if &address[0..2] == "0x" {
                address = (&address.clone()[2..address.len()]).to_string();
            }
        }
        let mut result : String ="0x".to_string();
        result.insert_str(2,address.as_str());
        result
    }else{
        panic!("invalid address");
    }
}
pub fn is_predefined_blocknumber (blocknumber : &str)->bool{
    blocknumber == "latest" || blocknumber == "pending" || blocknumber == "earliest"
}

pub fn input_blocknumber_formatter (blocknumber : String)->String {
    let mut result: String =String::new();
    let clone = blocknumber.clone();
    if blocknumber.clone() == "undefined".to_string() {
        result = "undefined".to_string();
    }else if is_predefined_blocknumber(clone.as_str()){
        result = blocknumber.clone();
    }else{
        result = String::to_hex(blocknumber.clone());
    }
    result
}
// var inputDefaultBlockNumberFormatter = function (blockNumber) {
//     if (blockNumber === undefined) {
//         return config.defaultBlock;
//     }
//     return inputBlockNumberFormatter(blockNumber);
// };
pub fn input_default_blocknumber_formatter(blocknumber: String)->String{
    let mut result : String = String::new();
    if blocknumber.clone() ==  "undefined".to_string(){
        result =config::config.defaultBlock.to_string();
    }else{
        result = input_blocknumber_formatter(blocknumber)
    }
    result
}
