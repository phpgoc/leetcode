#[test]
fn is_power_of_four() {
    use super::two;
    assert!(two::is_power_of_four(4));
}
#[test]
fn integer_break() {
    use super::three;
    assert_eq!(three::integer_break(10), 36);
}
