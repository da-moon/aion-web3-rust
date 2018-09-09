#[derive(Debug)]
pub struct Options {
    pub encoding: &'static str,
}

pub fn hash(value: &str , option: Options ) -> &str{
    let mut result: &str = value; 
    if option.encoding == "hex"{
         if value.len() > 2 &&  &value[0..2] == "0x" {
            result = &value[2..value.len()]  ;      
        };
    };
    result
}



// module.exports = function (value, options) {
//     if (options && options.encoding === 'hex') {
//         if (value.length > 2 && value.substr(0, 2) === '0x') {
//             value = value.substr(2);
//         }
//         value = CryptoJS.enc.Hex.parse(value);
//     }

//     return sha3(value, {
//         outputLength: 256
//     }).toString();
// };