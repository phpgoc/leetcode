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
