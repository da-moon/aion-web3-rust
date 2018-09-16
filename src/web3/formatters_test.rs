
#[cfg(test)]
pub mod formatters_test {
    use web3::formatters;
    #[test]
    #[ignore]
    fn input_address_formatter() {
        let input : String = "0xA00983F07C11eE9160a64dd3bA3dc3D1F88332A2869f25725F56Cbd0Be32EF7a".to_string();
        let result : String = "0xA00983F07C11eE9160a64dd3bA3dc3D1F88332A2869f25725F56Cbd0Be32EF7a".to_string();
        assert_eq!(formatters::input_address_formatter(input),result );
    }

    // #[test]
    // #[ignore]
    // fn is_predefined_blocknumber_one(){
    //     let input = "latest";
    //     let result = true;
    //     assert_eq!(formatters::is_predefined_blocknumber(input),result );
    // }
    // #[test]
    // #[ignore]
    // fn is_predefined_blocknumber_two(){
    //     let input = "pending";
    //     let result = true;
    //     assert_eq!(formatters::is_predefined_blocknumber(input),result );
    // }
    // #[test]
    // #[ignore]
    // fn is_predefined_blocknumber_three(){
    //     let input = "earliest";
    //     let result = true;
    //     assert_eq!(formatters::is_predefined_blocknumber(input),result );
    // }
    // #[test]
    // #[ignore]
    // fn is_predefined_blocknumber_false(){
    //     let input = "Something something blockchain";
    //     let result = false;
    //     assert_eq!(formatters::is_predefined_blocknumber(input),result );
    // }
    // The Times 03/Jan/2009 Chancellor on brink of second bailout for banks
    #[test]
    #[ignore]

    fn input_blocknumber_formatter_undefined() {
        let input = "undefined".to_string();
        let result = "undefined".to_string();
        assert_eq!(formatters::input_blocknumber_formatter(input),result );
    }
    #[test]
    #[ignore]

    fn input_blocknumber_formatter_predefined() {
        let input = "pending".to_string();
        let result = "pending".to_string();
        assert_eq!(formatters::input_blocknumber_formatter(input),result );
    }
    #[test]
    #[ignore]

    fn input_blocknumber_formatter() {
        let input = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks".to_string();
        let result = "0x5468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73".to_string();
        assert_eq!(formatters::input_blocknumber_formatter(input),result );
    }
}
