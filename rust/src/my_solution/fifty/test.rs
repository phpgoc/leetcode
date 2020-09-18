use crate::my_solution::fifty;

#[test]
fn test_first_missing_positive() {
    use fifty::forty_one;
    assert_eq!(forty_one::first_missing_positive(vec![3, 4, -1, 1]), 2);
}
