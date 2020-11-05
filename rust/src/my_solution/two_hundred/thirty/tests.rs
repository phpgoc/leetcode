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
    assert_eq!(three::count_digit_one(13), 6);
    assert_eq!(three::count_digit_one(301203), 250645);
}
#[test]
fn product_except_self() {
    use thirty::eight;
    assert_eq!(
        eight::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
}

#[test]
fn max_sliding_window() {
    use thirty::nine;
    assert_eq!(
        nine::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
    assert_eq!(
        nine::max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5),
        vec![10, 10, 9, 2]
    );
}
