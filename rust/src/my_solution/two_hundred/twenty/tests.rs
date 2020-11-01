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
