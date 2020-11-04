use crate::my_solution::two_hundred::thirty;

#[test]
fn is_power_of_two() {
    use thirty::one;
    assert!(one::is_power_of_two(1));
    assert!(one::is_power_of_two(16));
    assert!(!one::is_power_of_two(218));
}