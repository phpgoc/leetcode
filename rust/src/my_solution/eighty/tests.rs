use crate::my_solution::eighty;

#[test]
fn test_remove_duplicates() {
    let mut input = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(eighty::eighty::remove_duplicates(&mut input), 5);
    assert_eq!(input, vec![1, 1, 2, 2, 3]);
}
