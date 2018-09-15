use super::super::utils::utils;
// var outputBigNumberFormatter = function (number) {
//     return utils.toBigNumber(number);
// };

// var inputAddressFormatter = function (address) {
//     if (utils.isStrictAddress(address)) {
//         return address;
//     } else if (utils.isAddress(address)) {
//         return '0x' + address;
//     }
//     throw new Error('invalid address');
// };

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