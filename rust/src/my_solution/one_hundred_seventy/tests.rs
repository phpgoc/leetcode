use crate::my_solution::one_hundred_seventy;
#[test]
fn test_title_to_number() {
    assert_eq!(
        one_hundred_seventy::one_hundred_seventy::title_to_number(String::from("ZY")),
        701
    );
}
