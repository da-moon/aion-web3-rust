
#[cfg(test)]
pub mod utils_test {
    use utils::utils;
    #[test]
    #[ignore]
    fn pad_left_no_char() {
        let case = "000123456".to_string();
        let result = utils::pad_left("123456".to_string(),3,String::new());
        assert_eq!(case, result);

    }
    #[test]
    #[ignore]
    fn pad_left() {
        let case = "xx123456".to_string();
        let result = utils::pad_left("123456".to_string(),2,"x".to_string());
        assert_eq!(case, result);

    }
    #[test]
    #[ignore]
    fn pad_right() {
        let case = "123456xx".to_string();
        let result = utils::pad_right("123456".to_string(),2,"x".to_string());
        assert_eq!(case, result);

    }
    #[test]
    #[ignore]
    fn pad_right_no_char() {
        let case = "123456000".to_string();
        let result = utils::pad_right("123456".to_string(),3,String::new());
        assert_eq!(case, result);

    }
    #[test]
    #[ignore]
    fn to_utf8(){
        let case = "JKL".to_string();
        let result = utils::to_utf8("4a4b4c".to_string());
        assert_eq!(case, result);
    }
    #[test]
    #[ignore]
    fn to_utf8_alt(){
        let case = "J".to_string();
        let result = utils::to_utf8("4aF".to_string());
        assert_eq!(case, result);
    }
    #[test]
    #[ignore]
    fn to_utf8_with_0x(){
        let case = "JKL".to_string();
        let result = utils::to_utf8("0x4a4b4c".to_string());
        assert_eq!(case, result);
    }
    #[test]
    #[ignore]
    fn to_utf8_odd_len(){
        let case = "JK".to_string();
        let result = utils::to_utf8("0x4a4b4".to_string());
        assert_eq!(case, result);
    }
    #[test]
    #[ignore]
   fn to_utf8_empty(){
        let case =String::new();
        let result = utils::to_utf8(String::new());
        assert_eq!(case, result);
    }
    #[test]    
    #[ignore]
    fn to_utf8_smaller_than_2_len(){
        let case = String::new();
        let result = utils::to_utf8("1".to_string());
        assert_eq!(case, result);
    }
}