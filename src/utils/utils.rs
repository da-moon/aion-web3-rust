// var padLeft = function (string, chars, sign) {
//     return new Array(chars - string.length + 1).join(sign ? sign : "0") + string;
// };
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
