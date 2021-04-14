#[test]
fn construct_rectangle() {
    use super::two;
    assert_eq!(two::construct_rectangle(4), vec![2, 2]);
    assert_eq!(two::construct_rectangle(55), vec![11, 5]);
    assert_eq!(two::construct_rectangle(1), vec![1, 1]);
    assert_eq!(two::construct_rectangle(6619 * 2), vec![6619, 2]);
}
