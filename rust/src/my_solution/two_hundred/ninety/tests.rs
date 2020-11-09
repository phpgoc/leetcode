#[test]
fn word_pattern() {
    use super::word_pattern;
    assert!(word_pattern(
        String::from("abba"),
        String::from("dog cat cat dog")
    ));
}

#[test]
fn can_win_nim() {
    use super::one;
    assert!(!one::can_win_nim(4));
}
