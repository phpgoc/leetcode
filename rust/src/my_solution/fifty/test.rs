use crate::my_solution::fifty;

#[test]
fn test_first_missing_positive() {
    use fifty::forty_one;
    assert_eq!(forty_one::first_missing_positive(vec![3, 4, -1, 1]), 2);
}

#[test]
fn test_trap() {
    use fifty::forty_two;
    assert_eq!(forty_two::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(forty_two::trap(vec![4, 2, 3]), 1);
    assert_eq!(forty_two::trap(vec![4, 2, 0, 3, 2, 4, 3, 4]), 10);
    assert_eq!(forty_two::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}

#[test]
fn test_multiply() {
    use fifty::forty_three;
    //    assert_eq!(
    //        forty_three::multiply(String::from("3"), String::from("2")),
    //        String::from("6")
    //    );
    assert_eq!(
        forty_three::multiply(String::from("123"), String::from("456")),
        String::from("56088")
    );
}

#[test]
fn test_jump() {
    use fifty::forty_five;
    assert_eq!(forty_five::jump(vec![2, 3, 1, 1, 4]), 2);
}
