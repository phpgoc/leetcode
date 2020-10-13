use crate::my_solution::one_hundred_thirty;

#[test]
fn test_solve() {
    let mut input = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    let expect = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    one_hundred_thirty::one_hundred_thirty::solve(&mut input);
    assert_eq!(input, expect);
}

#[test]
fn test_partition() {
    use one_hundred_thirty::one_hundred_thirty_one;
    let expect = vec![
        vec![String::from("aa"), String::from("b")],
        vec![String::from("a"), String::from("a"), String::from("b")],
    ];
    let result = one_hundred_thirty_one::partition(String::from("aab"));
    assert_eq!(result.len(), expect.len());
    for i in expect {
        assert!(result.contains(&i));
    }
}
