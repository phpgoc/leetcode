#[test]
fn word_pattern() {
    use super::word_pattern;
    assert!(word_pattern(
        String::from("abba"),
        String::from("dog cat cat dog")
    ));
}
