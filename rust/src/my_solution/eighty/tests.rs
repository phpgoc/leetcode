use crate::my_solution::eighty;

#[test]
fn test_remove_duplicates() {
    let mut input = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(eighty::eighty::remove_duplicates(&mut input), 5);
    assert_eq!(input, vec![1, 1, 2, 2, 3]);
}

#[test]
fn test_search() {
    use eighty::eighty_one;
    assert!(eighty_one::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    assert!(eighty_one::search(vec![3, 5, 1], 1));
    assert!(eighty_one::search(vec![1, 1, 3, 1], 3));
    assert!(eighty_one::search(vec![0, 0, 1, 1, 2, 0], 2));
}
