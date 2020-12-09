#[test]
fn last_remaining() {
    assert_eq!(super::last_remaining(7), 4);
    assert_eq!(super::last_remaining(9), 6);
}
#[test]
fn is_subsequence() {
    use super::two;
    assert!(two::is_subsequence(
        String::from("abc"),
        String::from("ahbgdc")
    ));
    assert!(!two::is_subsequence(
        String::from("axc"),
        String::from("ahbgdc")
    ));
    assert!(two::is_subsequence(String::from("a"), String::from("a")));
}
#[test]
fn valid_utf8() {
    use super::three;
    assert!(three::valid_utf8(vec![197, 130, 1]));
    assert!(!three::valid_utf8(vec![235, 140, 4]));
}
