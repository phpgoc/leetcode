use crate::my_solution::one_hundred_twenty;

#[test]
fn test_minimum_total() {
    let input = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    assert_eq!(
        one_hundred_twenty::one_hundred_twenty::minimum_total(input),
        11
    );
}
