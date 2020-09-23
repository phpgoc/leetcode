use crate::my_solution::sixty;
use crate::my_solution::sixty::sixty::get_permutation;

#[test]
fn test_get_permutation() {
    assert_eq!(sixty::sixty::get_permutation(3, 2), String::from("132"));
    assert_eq!(sixty::sixty::get_permutation(3, 3), String::from("213"));
    assert_eq!(sixty::sixty::get_permutation(4, 9), String::from("2314"));
}
