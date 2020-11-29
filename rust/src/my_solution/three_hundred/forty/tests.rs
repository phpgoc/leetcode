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

#[test]
fn reverse_string() {
    use super::four;
    let mut input = ['h', 'e', 'l', 'l', 'o'].to_vec();
    four::reverse_string(&mut input);
    assert_eq!(input, ['o', 'l', 'l', 'e', 'h'].to_vec());
}
