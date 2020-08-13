use crate::my_solution::ten::two::add_two_numbers;

#[test]
fn test_two_sum() {
    let nums = vec![2, 3, 5, 7];
    let targat = 9;
    let answer = crate::my_solution::ten::one::two_sum(nums, targat);
    assert_eq!(answer, vec![0, 3])
}

#[test]
fn test_add_two_numbers() {
    use crate::my_solution::ten::two;

    let l1 = Some(Box::new(two::ListNode {
        val: 2,
        next: Some(Box::new(two::ListNode {
            val: 4,
            next: Some(Box::new(two::ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Some(Box::new(two::ListNode {
        val: 5,
        next: Some(Box::new(two::ListNode {
            val: 6,
            next: Some(Box::new(two::ListNode { val: 4, next: None })),
        })),
    }));
    let result = Some(Box::new(two::ListNode {
        val: 7,
        next: Some(Box::new(two::ListNode {
            val: 0,
            next: Some(Box::new(two::ListNode { val: 8, next: None })),
        })),
    }));
    assert_eq!(add_two_numbers(l1, l2), result);
}
