use crate::my_solution::one_hundred_fifty;

#[test]
fn test_eval_rpn() {
    assert_eq!(
        one_hundred_fifty::one_hundred_fifty::eval_rpn(vec![
            String::from("2"),
            String::from("1"),
            String::from("+"),
            String::from("3"),
            String::from("*"),
        ]),
        9
    );
}

#[test]
fn test_max_product() {
    use one_hundred_fifty::one_hundred_fifty_two;
    assert_eq!(one_hundred_fifty_two::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(one_hundred_fifty_two::max_product(vec![-2, 0, -1]), 0);
}
