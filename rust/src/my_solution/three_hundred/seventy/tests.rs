#[test]
fn super_pow() {
    use super::two;
    assert_eq!(two::super_pow(1, vec![2, 3, 4, 5, 6]), 1);
    assert_eq!(two::super_pow(2, vec![1, 0]), 1024);
    assert_eq!(two::super_pow(2147483647, vec![2, 0, 0]), 1198);
}
