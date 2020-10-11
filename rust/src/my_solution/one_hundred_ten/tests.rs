use crate::my_solution::one_hundred_ten;

#[test]
fn test_generate() {
    use one_hundred_ten::one_hundred_eighteen;
    let result = one_hundred_eighteen::generate(5);
    assert!(result.contains(vec![1, 4, 6, 4, 1].as_ref()));
}
