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
