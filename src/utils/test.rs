
#[cfg(test)]
mod tests {
    use utils::sha3;

    #[test]
    fn drop_0x() {
        let option = sha3::Options{
            encoding:"hex",
        };
        assert_eq!(sha3::hash("0x12345",option), "12345");
    }
    #[test]
    fn sha3_hash(){
        let option = sha3::Options{
            encoding:"hex",
        };
        assert_eq!(sha3::hash("0x12345",option), "12345");
    }
}
