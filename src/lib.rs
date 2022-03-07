pub mod base64;
mod err;

#[cfg(test)]
mod tests {
    use crate::base64::encode;

    #[test]
    fn test_encode_empty() {
        let result = encode("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_encode_f() {
        let result = encode("f").unwrap();
        assert_eq!(result, "Zg==");
    }

    #[test]
    fn test_encode_fo() {
        let result = encode("fo").unwrap();
        assert_eq!(result, "Zm8=");
    }

    #[test]
    fn test_encode_foo() {
        let result = encode("foo").unwrap();
        assert_eq!(result, "Zm9v");
    }

    #[test]
    fn test_encode_foob() {
        let result = encode("foob").unwrap();
        assert_eq!(result, "Zm9vYg==");
    }

    #[test]
    fn test_encode_fooba() {
        let result = encode("fooba").unwrap();
        assert_eq!(result, "Zm9vYmE=");
    }

    #[test]
    fn test_encode_foobar() {
        let result = encode("foobar").unwrap();
        assert_eq!(result, "Zm9vYmFy");
    }
}
