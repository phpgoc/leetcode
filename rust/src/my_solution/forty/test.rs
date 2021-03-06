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

#[test]
fn test_search() {
    use forty::thirty_three;
    assert_eq!(thirty_three::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(thirty_three::search(vec![1], 0), -1);
    assert_eq!(thirty_three::search(vec![1, 3], 0), -1);
    assert_eq!(thirty_three::search(vec![1, 3], 3), 1);
}

#[test]
fn test_search_range() {
    use forty::thirty_four;
    assert_eq!(
        thirty_four::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
    assert_eq!(
        thirty_four::search_range(vec![5, 7, 7, 8, 8, 10], 5),
        vec![0, 0]
    );
    assert_eq!(thirty_four::search_range(vec![], 0), vec![-1, -1]);
    assert_eq!(thirty_four::search_range(vec![2, 2], 2), vec![0, 1]);
    assert_eq!(thirty_four::search_range(vec![1, 1, 2], 1), vec![0, 1]);
}

#[test]
fn test_search_insert() {
    use forty::thirty_five;
    assert_eq!(thirty_five::search_insert(vec![1, 3, 5, 6], 5), 2);
}

#[test]
fn test_is_valid_sudoku() {
    use forty::thirty_six;
    assert_eq!(
        thirty_six::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]),
        true
    );
    assert_eq!(
        thirty_six::is_valid_sudoku(vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.']
        ]),
        false
    );
}

#[test]
fn test_count_and_say() {
    use forty::thirty_eight;
    assert_eq!(thirty_eight::count_and_say(5), String::from("111221"));
}

#[test]
fn test_combination_sum() {
    use forty::thirty_nine;
    let input = vec![2, 3, 6, 7];
    let result = thirty_nine::combination_sum(input, 7);
    let expect = vec![vec![7], vec![2, 2, 3]];
    for i in expect {
        assert!(result.contains(&i));
    }
    //    println!("{:?}", result);
}

#[test]
fn test_combination_sum2() {
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    let expect = vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]];
    let result = forty::forty::combination_sum2(candidates, target);
    for i in expect {
        assert!(result.contains(&i));
    }
}
