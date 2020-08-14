use crate::my_solution::twenty;

#[test]
fn test_max_area() {
    use twenty::eleven;
    assert_eq!(eleven::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(eleven::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
}
