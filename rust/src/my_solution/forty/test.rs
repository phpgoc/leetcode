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

#[test]
fn test_longest_valid_parentheses() {
    use forty::thirty_two;
    assert_eq!(
        thirty_two::longest_valid_parentheses(String::from("(()")),
        2
    );
    assert_eq!(
        thirty_two::longest_valid_parentheses(String::from("()(()")),
        2
    );
    assert_eq!(
        thirty_two::longest_valid_parentheses(String::from(")()())()()(")),
        4
    );
    assert_eq!(
        thirty_two::longest_valid_parentheses(String::from(")(((((()())()()))()(()))(")),
        22
    );
    assert_eq!(
        thirty_two::longest_valid_parentheses(String::from("(())()(()((")),
        6
    );
}
