pub mod base64;
mod err;

#[cfg(test)]
mod tests {
    use crate::base64::{decode, encode};

    #[test]
    fn test_empty() {
        let encoded = encode("").unwrap();
        assert_eq!(encoded, "");
        let decoded = decode("").unwrap();
        assert_eq!(decoded, "");
    }

    #[test]
    fn test_f() {
        let encoded = encode("f").unwrap();
        assert_eq!(encoded, "Zg==");
        let decoded = decode("Zg==").unwrap();
        assert_eq!(decoded, "f");
    }

    #[test]
    fn test_fo() {
        let encoded = encode("fo").unwrap();
        assert_eq!(encoded, "Zm8=");
        let decoded = decode("Zm8=").unwrap();
        assert_eq!(decoded, "fo");
    }

    #[test]
    fn test_foo() {
        let encoded = encode("foo").unwrap();
        assert_eq!(encoded, "Zm9v");
        let decoded = decode("Zm9v").unwrap();
        assert_eq!(decoded, "foo");
    }

    #[test]
    fn test_foob() {
        let encoded = encode("foob").unwrap();
        assert_eq!(encoded, "Zm9vYg==");
        let decoded = decode("Zm9vYg==").unwrap();
        assert_eq!(decoded, "foob");
    }

    #[test]
    fn test_fooba() {
        let encoded = encode("fooba").unwrap();
        assert_eq!(encoded, "Zm9vYmE=");
        let decoded = decode("Zm9vYmE=").unwrap();
        assert_eq!(decoded, "fooba");
    }

    #[test]
    fn test_foobar() {
        let encoded = encode("foobar").unwrap();
        assert_eq!(encoded, "Zm9vYmFy");
        let decoded = decode("Zm9vYmFy").unwrap();
        assert_eq!(decoded, "foobar");
    }
}
