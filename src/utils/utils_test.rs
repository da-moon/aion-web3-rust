
#[cfg(test)]
pub mod utils_test {
    use utils::utils;
    use utils::utils::ToHex;
    // #[test]
    // #[ignore]
    // fn pad_left_no_char() {
    //     let result = "000123456".to_string();
    //     let input = utils::pad_left("123456".to_string(),3,String::new());
    //     assert_eq!(result, input);

    // }
    // #[test]
    // #[ignore]
    // fn pad_left() {
    //     let result = "xx123456".to_string();
    //     let input = utils::pad_left("123456".to_string(),2,"x".to_string());
    //     assert_eq!(result, input);

    // }
    // #[test]
    // #[ignore]
    // fn pad_right() {
    //     let result = "123456xx".to_string();
    //     let input = utils::pad_right("123456".to_string(),2,"x".to_string());
    //     assert_eq!(result, input);

    // }
    // #[test]
    // #[ignore]
    // fn pad_right_no_char() {
    //     let result = "123456000".to_string();
    //     let input = utils::pad_right("123456".to_string(),3,String::new());
    //     assert_eq!(result, input);

    // }
    // #[test]
    // #[ignore]
    // fn to_ascii(){
    //     let result = "JKL".to_string();
    //     let input = utils::to_ascii("4a4b4c".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_ascii_alt(){
    //     let result = "J".to_string();
    //     let input = utils::to_ascii("4aF".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_ascii_with_0x(){
    //     let result = "JKL".to_string();
    //     let input = utils::to_ascii("0x4a4b4c".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_ascii_odd_len(){
    //     let result = "JK".to_string();
    //     let input = utils::to_ascii("0x4a4b4".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_ascii_empty(){
    //     let result =String::new();
    //     let input = utils::to_ascii(String::new());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_ascii_smaller_than_2_len(){
    //     let result = String::new();
    //     let input = utils::to_ascii("1".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_utf8(){
    //     let result = "JKL".to_string();
    //     let input = utils::to_utf8("4a4b4c".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_utf8_alt(){
    //     let result = "Hello world!".to_string();
    //     let input = utils::to_utf8("48656c6c6f20776f726c6421".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_utf8_with_0x(){
    //     let result = "JKL".to_string();
    //     let input = utils::to_utf8("0x4a4b4c".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_utf8_odd_len(){
    //     let result = "JK".to_string();
    //     let input = utils::to_utf8("0x4a4b4".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_utf8_empty(){
    //     let result =String::new();
    //     let input = utils::to_utf8(String::new());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn to_utf8_smaller_than_2_len(){
    //     let result = String::new();
    //     let input = utils::to_utf8("1".to_string());
    //     assert_eq!(result, input);
    // }
    // #[test]
    // #[ignore]
    // fn from_utf8_odd_len(){
    //     let result = "0x4a4b".to_string();
    //     let input = utils::from_utf8("JK".to_string());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn from_utf8_empty(){
    //     let result ="0x".to_string();
    //     let input = utils::from_utf8(String::new());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn from_ascii(){
    //     let result = "0x4920686176652031303021".to_string();
    //     let input = utils::from_ascii("I have 100!".to_string());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn from_ascii_alt(){
    //     let result = "0x48656c6c6f20776f726c6421".to_string();
    //     let input = utils::from_ascii("Hello world!".to_string());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn from_ascii_empty(){
    //     let result ="0x".to_string();
    //     let input = utils::from_ascii(String::new());
    //     assert_eq!(input, result);
    // }

    // #[test]
    // #[ignore]
    // fn to_decimal(){
    //     let result = "234".to_string();
    //     let input = utils::to_decimal("0xea".to_string());
    //     assert_eq!(input, result);
    // }
    #[test]
    fn from_decimal_positive() {
        let input = "16".to_string();
        let result = "0x10".to_string();
        assert_eq!(utils::from_decimal(input),result);
    }
    #[test]
    fn from_decimal_negative() {
        let input = "-11".to_string();
        let result = "-0xb".to_string();
        assert_eq!(utils::from_decimal(input),result);
    }
    // #[test]
    // #[ignore]
    // fn to_big_number_pos_hex(){
    //     let result = "15".to_string();
    //     let input = utils::to_big_number("0xF".to_string());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn to_big_number_neg_hex(){
    //     let result = "-15".to_string();
    //     let input = utils::to_big_number("-0xF".to_string());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn to_big_number_pos_dec(){
    //     let result = "15".to_string();
    //     let input = utils::to_big_number("15".to_string());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn to_big_number_neg_dec(){
    //     let result = "-15".to_string();
    //     let input = utils::to_big_number("-15".to_string());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn to_big_number_no_0x_hex(){
    //     let result = String::new();
    //     let input = utils::to_big_number("F".to_string());
    //     assert_eq!(input, result);
    // }
    // #[test]
    // #[ignore]
    // fn is_strict_address_success(){
    //     let input = "0xa00983f07c11ee9160a64dd3ba3dc3d1f88332a2869f25725f56cbd0be32ef7a".to_string();
    //     let result = true;
    //     assert_eq!(utils::is_strict_address(input), result);
    // }
    // #[test]
    // #[ignore]
    // fn is_strict_address_fail(){
    //     let input = "0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe".to_string();
    //     let result = false;
    //     assert_eq!(utils::is_strict_address(input), result);
    // }
    // #[test]
    // #[ignore]
    // fn is_checksum_address(){
    //     let input = "0XA00983F07C11EE9160A64DD3BA3DC3D1F88332A2869F25725F56CBD0BE32EF7A".to_string();
    //     let result = false;
    //     assert_eq!(utils::is_checksum_address(input), result);
    // }
    // #[test]
    // #[ignore]
    //  fn is_checksum_address_fail(){
    //     let input = "0xA00983F07C11eE9160a64dd3bA3dc3D1F88332A2869f25725F56Cbd0Be32EF7a".to_string();
    //     let result = true;
    //     assert_eq!(utils::is_checksum_address(input), result);

    // }
    // #[test]
    // #[ignore]
    //     fn to_checksum_address(){
    //     let input = "0XA00983F07C11EE9160A64DD3BA3DC3D1F88332A2869F25725F56CBD0BE32EF7A".to_string();
    //     let result = "0xA00983F07C11eE9160a64dd3bA3dc3D1F88332A2869f25725F56Cbd0Be32EF7a".to_string();
    //     assert_eq!(utils::to_checksum_address(input), result);
    // }
    // #[test]
    // #[ignore]
    //     fn is_address(){
    //     let input = "0xA00983F07C11eE9160a64dd3bA3dc3D1F88332A2869f25725F56Cbd0Be32EF7a".to_string();
    //     let result = true;
    //     assert_eq!(utils::is_address(input), result);
    // }
    // #[test]
    // #[ignore]
    //     fn is_address_fail(){
    //     let input = "0XA00983F07C11EE9160A64DD3BA3DC3D1F88332A2869F25725F56CBD0BE32EF7A".to_string();
    //     let result = false;
    //     assert_eq!(utils::is_address(input), result);
    // }
    #[test]
    fn to_hex_bool() {
        let input = true;
        let result = "test".to_string();
        assert_eq!(bool::to_hex(input), result);
    }
}
