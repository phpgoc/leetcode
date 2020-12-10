#[test]
fn find_nth_digit() {
    assert_eq!(super::find_nth_digit(11), 0);
    assert_eq!(super::find_nth_digit(3), 3);
    assert_eq!(super::find_nth_digit(10), 1);
    assert_eq!(super::find_nth_digit(9), 9);
}
