use crate::my_solution::thirty;
#[test]
fn test_merge_two_lists() {
    use thirty::twenty_one;
    let l1 = Some(Box::new(twenty_one::ListNode {
        val: 1,
        next: Some(Box::new(twenty_one::ListNode { val: 3, next: None })),
    }));
    let l2 = Some(Box::new(twenty_one::ListNode {
        val: 2,
        next: Some(Box::new(twenty_one::ListNode { val: 4, next: None })),
    }));
    let result = Some(Box::new(twenty_one::ListNode {
        val: 1,
        next: Some(Box::new(twenty_one::ListNode {
            val: 2,
            next: Some(Box::new(twenty_one::ListNode {
                val: 3,
                next: Some(Box::new(twenty_one::ListNode { val: 4, next: None })),
            })),
        })),
    }));
    assert_eq!(twenty_one::merge_two_lists(l1, l2), result);
}

#[test]
fn test_generate_parenthesis() {
    use thirty::twenty_two;
    let result = twenty_two::generate_parenthesis(3);
    let expect = vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    for i in expect {
        assert!(result.contains(&i));
    }
}

#[test]
fn test_merge_k_lists() {
    use thirty::twenty_three;
    let l1 = Some(Box::new(twenty_three::ListNode {
        val: 1,
        next: Some(Box::new(twenty_three::ListNode { val: 3, next: None })),
    }));
    let l2 = Some(Box::new(twenty_three::ListNode {
        val: 2,
        next: Some(Box::new(twenty_three::ListNode { val: 4, next: None })),
    }));
    let expect = Some(Box::new(twenty_three::ListNode {
        val: 1,
        next: Some(Box::new(twenty_three::ListNode {
            val: 2,
            next: Some(Box::new(twenty_three::ListNode {
                val: 3,
                next: Some(Box::new(twenty_three::ListNode { val: 4, next: None })),
            })),
        })),
    }));
    let input = vec![l1, l2];
    let result = twenty_three::merge_k_lists(input);
    assert_eq!(expect, result);
}

#[test]
fn test_swap_pairs() {
    use thirty::twenty_four;
    let expect = Some(Box::new(twenty_four::ListNode {
        val: 1,
        next: Some(Box::new(twenty_four::ListNode {
            val: 2,
            next: Some(Box::new(twenty_four::ListNode { val: 3, next: None })),
        })),
    }));
    let input = Some(Box::new(twenty_four::ListNode {
        val: 2,
        next: Some(Box::new(twenty_four::ListNode {
            val: 1,
            next: Some(Box::new(twenty_four::ListNode { val: 3, next: None })),
        })),
    }));
    assert_eq!(twenty_four::swap_pairs(input), expect);
}

#[test]
fn test_remove_duplicates() {
    use thirty::twenty_six;
    let mut input = vec![1, 1, 2];
    assert_eq!(twenty_six::remove_duplicates(input.as_mut()), 2);
    assert_eq!(input, vec![1, 2]);
}
