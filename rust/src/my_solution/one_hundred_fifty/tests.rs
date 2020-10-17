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

#[test]
fn test_min_stack() {
    use one_hundred_fifty::one_hundred_fifty_five::MinStack;
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3); //--> 返回 -3.
    min_stack.pop();
    assert_eq!(min_stack.top(), 0); //--> 返回 0.
    assert_eq!(min_stack.get_min(), -2); //--> 返回 -2.
}
