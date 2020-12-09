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
#[test]
fn decode_string() {
    use super::four;
    assert_eq!(
        four::decode_string(String::from("3[a]2[bc]")),
        String::from("aaabcbc")
    );
    assert_eq!(
        four::decode_string(String::from("abc3[cd]xyz")),
        String::from("abccdcdcdxyz")
    );
    assert_eq!(
        four::decode_string(String::from("3[a2[c]]")),
        String::from("accaccacc")
    );
}
#[test]
fn longest_substring() {
    use super::five;
    assert_eq!(five::longest_substring(String::from("ababbc"), 2), 5);
    assert_eq!(five::longest_substring(String::from("aaabb"), 3), 3);
    assert_eq!(five::longest_substring(String::from("ababacb"), 3), 0);
    assert_eq!(five::longest_substring(String::from("weitong"), 2), 0);
}
