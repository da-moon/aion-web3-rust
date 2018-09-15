use crypto::digest::Digest;
use crypto::sha3::Sha3;
#[derive(Debug)]
pub struct Options <'a>  {
    pub encoding: &'a str,
}
impl<'a> Options<'a> {
    pub fn hash(&self,value: &'a str  ) -> String{
        let mut result :  &'a str  = value; 
        let mut hasher = Sha3::sha3_256();
        if self.encoding == "hex"{
            if value.len() > 2 &&  &value[0..2] == "0x" {
                result = &value[2..value.len()]  ;      
            };
        };
        hasher.input_str(result);
        hasher.result_str()
    }
}
pub fn hash(address: String  ) -> String{
    let mut result :  &str  = address.as_str(); 
    let mut hasher = Sha3::keccak256();
    if address.len() > 2 &&  &address[0..2] == "0x" {
        result = &address[2..address.len()]  ;      
    };
    hasher.input_str(result);
    hasher.result_str().to_string()
}
