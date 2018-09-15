
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
    #[test]
    #[ignore]
    fn sha3_hash_alt() {
        let input = String::new();
        let result ="c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470".to_string();
        println!("{:?}",result);
        assert_eq!(sha3::hash(input), result);

    }
     #[test]
    #[ignore]
     fn sha3_hash_alt2() {
        let input = "0xA00983F07C11EE9160A64DD3BA3DC3D1F88332A2869F25725F56CBD0BE32EF7A".to_string();
        let result ="76562b624c806598d276b2c68999de0a40ec0c9dc691935a36cb16a1d4b3b5f1".to_string();
        assert_eq!(sha3::hash(input), result);

    }
    #[test]
    #[ignore]
      fn sha3_hash_alt3() {
        let input = "0xa".to_string();
        let result ="3ac225168df54212a25c1c01fd35bebfea408fdac2e31ddd6f80a4bbf9a5f1cb".to_string();
        assert_eq!(sha3::hash(input), result);

    }

}