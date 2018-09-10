
#[cfg(test)]
pub mod sha3_test {
    use utils::sha3;

    #[test]
    #[ignore]
    fn sha3_hash() {
        let option = sha3::Options{
            encoding:"hex",
        };
        let case = option.hash("0xabc");
        assert_eq!(case, "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532".to_string());

    }
}