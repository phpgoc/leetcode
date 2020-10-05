use crate::my_solution::ninety;

#[test]
fn test_subsets_with_dup() {
    let result = ninety::ninety::subsets_with_dup(vec![1, 2, 2]);
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
    let result = ninety::ninety::subsets_with_dup(vec![5, 5, 5, 5, 5]);
}

#[test]
fn test_num_decodings() {
    use ninety::ninety_one;
    assert_eq!(ninety_one::num_decodings(String::from("12")), 2);
    assert_eq!(ninety_one::num_decodings(String::from("226")), 3);
    assert_eq!(
        ninety_one::num_decodings(String::from("2611055971756562")),
        4
    );
}
