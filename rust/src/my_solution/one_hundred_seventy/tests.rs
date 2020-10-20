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
