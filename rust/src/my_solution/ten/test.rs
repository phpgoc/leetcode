#[test]
fn test_two_sum() {
    let nums = vec![2, 3, 5, 7];
    let targat = 9;
    let answer = crate::my_solution::ten::one::two_sum(nums, targat);
    assert_eq!(answer, vec![0, 3])
}
