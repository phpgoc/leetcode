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
