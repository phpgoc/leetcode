use crate::my_solution::two_hundred;

#[test]
fn test_num_islands() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    assert_eq!(two_hundred::two_hundred::num_islands(grid), 1)
}

#[test]
fn test_range_bitwise_and() {
    use two_hundred::one;
    assert_eq!(one::range_bitwise_and(5, 7), 4);
}

#[test]
fn test_is_happy() {
    use two_hundred::two;
    assert!(two::is_happy(19));
    two::is_happy(2);
}
