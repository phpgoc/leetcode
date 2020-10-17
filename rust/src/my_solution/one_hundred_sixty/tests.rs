use crate::my_solution::one_hundred_sixty;
#[test]
fn test_find_peak_element() {
    assert_eq!(
        one_hundred_sixty::one_hundred_sixty::find_peak_element(vec![1, 2, 3, 1]),
        2
    );
}

#[test]
fn test_maximum_gap() {
    use one_hundred_sixty::one_hundred_sixty_four;
    assert_eq!(one_hundred_sixty_four::maximum_gap(vec![3, 6, 9, 1]), 3);
}
