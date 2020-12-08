#[test]
fn last_remaining() {
    assert_eq!(super::last_remaining(7), 4);
    assert_eq!(super::last_remaining(9), 6);
}
