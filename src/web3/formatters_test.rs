
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
}
