use crate::my_solution::ninty;

#[test]
fn test_subsets_with_dup() {
    let result = ninty::ninty::subsets_with_dup(vec![1, 2, 2]);
    let expect = vec![
        vec![2],
        vec![1],
        vec![1, 2, 2],
        vec![2, 2],
        vec![1, 2],
        vec![],
    ];
    for i in expect {
        assert!(result.contains(&i));
    }
    let result = ninty::ninty::subsets_with_dup(vec![5, 5, 5, 5, 5]);
}
