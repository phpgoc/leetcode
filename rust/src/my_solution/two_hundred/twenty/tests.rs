use crate::my_solution::two_hundred::twenty;
#[test]
fn contains_nearby_almost_duplicate() {
    assert!(twenty::contains_nearby_almost_duplicate(
        vec![1, 2, 3, 1],
        3,
        0
    ));
    assert!(!twenty::contains_nearby_almost_duplicate(
        vec![1, 5, 9, 1, 5, 9],
        2,
        3
    ));
}

#[test]
fn maximal_square() {
    use twenty::one;
    // assert_eq!(
    //     one::maximal_square(vec![
    //         vec!['1', '0', '1', '0', '0'],
    //         vec!['1', '0', '1', '1', '1'],
    //         vec!['1', '1', '1', '1', '1'],
    //         vec!['1', '0', '0', '1', '0']
    //     ]),
    //     4
    // );
    assert_eq!(one::maximal_square(vec![vec!['1'],]), 1);
}
