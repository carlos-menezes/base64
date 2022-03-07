mod base64;
mod err;

#[cfg(test)]
mod tests {
    use crate::base64::base64_encode;

    #[test]
    fn test_len7() {
        let result = base64_encode("light w").unwrap();
        assert_eq!(result, "bGlnaHQgdw==");
    }

    #[test]
    fn test_len8() {
        let result = base64_encode("light wo").unwrap();
        assert_eq!(result, "bGlnaHQgd28=");
    }

    #[test]
    fn test_len9() {
        let result = base64_encode("light wor").unwrap();
        assert_eq!(result, "bGlnaHQgd29y");
    }
}
