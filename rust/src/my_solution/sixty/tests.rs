use crate::my_solution::sixty;
use crate::my_solution::sixty::sixty::get_permutation;

#[test]
fn test_get_permutation() {
    assert_eq!(sixty::sixty::get_permutation(3, 2), String::from("132"));
    assert_eq!(sixty::sixty::get_permutation(3, 3), String::from("213"));
    assert_eq!(sixty::sixty::get_permutation(4, 9), String::from("2314"));
}

#[test]
fn test_rotate_right() {
    use sixty::sixty_one;
    use sixty_one::ListNode;
    let input = Some(Box::from(ListNode {
        val: 1,
        next: Some(Box::from(ListNode {
            val: 2,
            next: Some(Box::from(ListNode { val: 3, next: None })),
        })),
    }));
    let expect = Some(Box::from(ListNode {
        val: 3,
        next: Some(Box::from(ListNode {
            val: 1,
            next: Some(Box::from(ListNode { val: 2, next: None })),
        })),
    }));
    assert_eq!(sixty_one::rotate_right(input, 1), expect);
}
