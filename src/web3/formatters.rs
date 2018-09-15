use super::super::utils::utils;
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

// var inputBlockNumberFormatter = function (blockNumber) {
//     if (blockNumber === undefined) {
//         return undefined;
//     } else if (isPredefinedBlockNumber(blockNumber)) {
//         return blockNumber;
//     }
//     return utils.toHex(blockNumber);
// };
pub fn input_blocknumber_formatter (blocknumber : String)->String {
    let mut result: String =   blocknumber.clone();
    if blocknumber == "undefined".to_string() {
        "undefined".to_string()
    }else if is_predefined_blocknumber(blocknumber.as_str()){
        blocknumber
    }else{
        blocknumber
    }
}