#[cfg(test)]
mod test_utils {
    use rectangular::{high_word, high_word_signed, low_word, low_word_signed};

    #[test]
    fn low_word_returns_lower_word() {
        assert_eq!(low_word!(0xDEADBEEF), 0xBEEF);
        assert_eq!(low_word!(0x0000FFFF), 65535);
    }

    #[test]
    fn high_word_returns_higher_word() {
        assert_eq!(high_word!(0xDEADBEEF), 0xDEAD);
        assert_eq!(high_word!(0xFFFF0000), 65535);
    }

    #[test]
    fn low_word_signed_returns_lower_word_with_sign() {
        assert_eq!(low_word_signed!(0x1234FFFF), -1);
        assert_eq!(low_word_signed!(0x12340010), 16);
    }

    #[test]
    fn high_word_signed_returns_higher_word_with_sign() {
        assert_eq!(high_word_signed!(0xFFFF1234), -1);
        assert_eq!(high_word_signed!(0x00101234), 16);
    }
}