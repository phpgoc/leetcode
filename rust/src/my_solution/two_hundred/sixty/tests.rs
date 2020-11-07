#[test]
fn single_number() {
    use super::single_number;
    let result = single_number(vec![1, 2, 1, 3, 2, 5]);
    let expect = vec![3, 5];
    assert_eq!(result.len(), expect.len());
    for i in expect {
        assert!(result.contains(&i));
    }
}
