#[test]
fn max_number() {
    use super::one;
    assert_eq!(
        one::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
        vec![9, 8, 6, 5, 3]
    );
    assert_eq!(
        one::max_number(vec![6, 7], vec![6, 0, 4], 5),
        vec![6, 7, 6, 0, 4]
    );
    assert_eq!(one::max_number(vec![8, 9], vec![3, 9], 3), vec![9, 8, 9]);
}

#[test]
fn coin_change() {
    use super::two;
    assert_eq!(two::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(two::coin_change(vec![186, 419, 83, 408], 6249), 20);
}