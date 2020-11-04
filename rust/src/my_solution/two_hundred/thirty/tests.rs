use crate::my_solution::two_hundred::thirty;

#[test]
fn is_power_of_two() {
    use thirty::one;
    assert!(one::is_power_of_two(1));
    assert!(one::is_power_of_two(16));
    assert!(!one::is_power_of_two(218));
}
#[test]
fn count_digit_one() {
    use thirty::three;
    assert_eq!(three::count_digit_one(13),6);
    assert_eq!(three::count_digit_one(301203),250645);
}