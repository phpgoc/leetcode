#[test]
fn original_digits() {
    use super::three;
    assert_eq!(
        three::original_digits(String::from("owoztneoer")),
        String::from("012")
    );
}
#[test]
fn character_replacement() {
    use super::four;
    assert_eq!(four::character_replacement(String::from("ABBA"), 2), 4);
    assert_eq!(four::character_replacement(String::from("AABABBA"), 1), 4);
}
