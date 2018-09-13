
#[cfg(test)]
pub mod utils_test {
    use utils::utils;
    #[test]
    #[ignore]
    fn pad_left_no_char() {
        let result = "000123456".to_string();
        let input = utils::pad_left("123456".to_string(),3,String::new());
        assert_eq!(result, input);

    }
    #[test]
    #[ignore]
    fn pad_left() {
        let result = "xx123456".to_string();
        let input = utils::pad_left("123456".to_string(),2,"x".to_string());
        assert_eq!(result, input);

    }
    #[test]
    #[ignore]
    fn pad_right() {
        let result = "123456xx".to_string();
        let input = utils::pad_right("123456".to_string(),2,"x".to_string());
        assert_eq!(result, input);

    }
    #[test]
    #[ignore]
    fn pad_right_no_char() {
        let result = "123456000".to_string();
        let input = utils::pad_right("123456".to_string(),3,String::new());
        assert_eq!(result, input);

    }
    #[test]
    #[ignore]
    fn to_ascii(){
        let result = "JKL".to_string();
        let input = utils::to_ascii("4a4b4c".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_ascii_alt(){
        let result = "J".to_string();
        let input = utils::to_ascii("4aF".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_ascii_with_0x(){
        let result = "JKL".to_string();
        let input = utils::to_ascii("0x4a4b4c".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_ascii_odd_len(){
        let result = "JK".to_string();
        let input = utils::to_ascii("0x4a4b4".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_ascii_empty(){
        let result =String::new();
        let input = utils::to_ascii(String::new());
        assert_eq!(result, input);
    }
    #[test]    
    #[ignore]
    fn to_ascii_smaller_than_2_len(){
        let result = String::new();
        let input = utils::to_ascii("1".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_utf8(){
        let result = "JKL".to_string();
        let input = utils::to_utf8("4a4b4c".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_utf8_alt(){
        let result = "Hello world!".to_string();
        let input = utils::to_utf8("48656c6c6f20776f726c6421".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_utf8_with_0x(){
        let result = "JKL".to_string();
        let input = utils::to_utf8("0x4a4b4c".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_utf8_odd_len(){
        let result = "JK".to_string();
        let input = utils::to_utf8("0x4a4b4".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn to_utf8_empty(){
        let result =String::new();
        let input = utils::to_utf8(String::new());
        assert_eq!(result, input);
    }
    #[test]   
    #[ignore]
    fn to_utf8_smaller_than_2_len(){
        let result = String::new();
        let input = utils::to_utf8("1".to_string());
        assert_eq!(result, input);
    }
    #[test]
    #[ignore]
    fn from_utf8_odd_len(){
        let result = "0x4a4b".to_string();
        let input = utils::from_utf8("JK".to_string());
        assert_eq!(input, result);
    }
    #[test]
    #[ignore]
    fn from_utf8_empty(){
        let result ="0x".to_string();
        let input = utils::from_utf8(String::new());
        assert_eq!(input, result);
    }
    #[test]
    #[ignore]
    fn from_ascii(){
        let result = "0x4920686176652031303021".to_string();
        let input = utils::from_ascii("I have 100!".to_string());
        assert_eq!(input, result);
    }
    #[test]
    #[ignore]
    fn from_ascii_alt(){
        let result = "0x48656c6c6f20776f726c6421".to_string();
        let input = utils::from_ascii("Hello world!".to_string());
        assert_eq!(input, result);
    }
    #[test]
    #[ignore]
    fn from_ascii_empty(){
        let result ="0x".to_string();
        let input = utils::from_ascii(String::new());
        assert_eq!(input, result);
    }

    #[test]   
    #[ignore]
    fn to_decimal(){
        let result = "234".to_string();
        let input = utils::to_decimal("0xea".to_string());
        assert_eq!(input, result);
    }
}