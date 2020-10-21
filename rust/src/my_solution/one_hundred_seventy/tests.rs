use crate::my_solution::one_hundred_seventy;
#[test]
fn test_title_to_number() {
    use one_hundred_seventy::one_hundred_seventy_one;
    assert_eq!(
        one_hundred_seventy_one::title_to_number(String::from("ZY")),
        701
    );
}

#[test]
fn test_railing_zeroes() {
    use one_hundred_seventy::one_hundred_seventy_two;
    assert_eq!(one_hundred_seventy_two::trailing_zeroes(30), 7);
}

#[test]
fn test_calculate_minimum_hp() {
    use one_hundred_seventy::one_hundred_seventy_four;
    assert_eq!(
        one_hundred_seventy_four::calculate_minimum_hp(vec![
            vec![-2, -3, 3],
            vec![-5, -10, 1],
            vec![10, 30, -5]
        ]),
        7
    );
}

#[test]
fn test_largest_number() {
    use one_hundred_seventy::one_hundred_seventy_nine;
    assert_eq!(
        one_hundred_seventy_nine::largest_number(vec![3, 30, 34, 5, 9]),
        String::from("9534330")
    );
    assert_eq!(
        one_hundred_seventy_nine::largest_number(vec![10, 2]),
        String::from("210")
    );
    assert_eq!(
        one_hundred_seventy_nine::largest_number(vec![432, 43243]),
        String::from("43243432")
    );
}
