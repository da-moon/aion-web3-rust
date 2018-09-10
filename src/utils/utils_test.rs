
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

}