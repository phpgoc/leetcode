use crate::my_solution::forty;

#[test]
fn test_next_permutation() {
    use forty::thirty_one;
    let mut vec = vec![1, 2, 3];
    thirty_one::next_permutation(vec.as_mut());
    assert_eq!(vec, vec![1, 3, 2]);
    let mut vec = vec![3, 2, 1];
    thirty_one::next_permutation(vec.as_mut());
    assert_eq!(vec, vec![1, 2, 3]);
    let mut vec = vec![1, 3, 2];
    thirty_one::next_permutation(vec.as_mut());
    assert_eq!(vec, vec![2, 1, 3]);
    let mut vec = vec![1, 5, 1];
    thirty_one::next_permutation(vec.as_mut());
    assert_eq!(vec, vec![5, 1, 1]);
}
