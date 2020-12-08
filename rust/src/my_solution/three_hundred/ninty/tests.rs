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
