use crate::my_solution::eighty;

#[test]
fn test_remove_duplicates() {
    let mut input = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(eighty::eighty::remove_duplicates(&mut input), 5);
    assert_eq!(input, vec![1, 1, 2, 2, 3]);
}

#[test]
fn test_search() {
    use eighty::eighty_one;
    assert!(eighty_one::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    assert!(eighty_one::search(vec![3, 5, 1], 1));
    assert!(eighty_one::search(vec![1, 1, 3, 1], 3));
    assert!(eighty_one::search(vec![0, 0, 1, 1, 2, 0], 2));
}

#[test]
fn test_delete_duplicates() {
    use eighty::eighty_two;
    use eighty_two::ListNode;
    let input = Some(Box::from(ListNode {
        val: 1,
        next: Some(Box::from(ListNode {
            val: 1,
            next: Some(Box::from(ListNode { val: 2, next: None })),
        })),
    }));
    assert_eq!(
        eighty_two::delete_duplicates(input),
        Some(Box::from(ListNode { val: 2, next: None }))
    );
}

#[test]
fn test_delete_duplicates2() {
    use eighty::eighty_three;
    use eighty_three::ListNode;
    let input = Some(Box::from(ListNode {
        val: 1,
        next: Some(Box::from(ListNode {
            val: 1,
            next: Some(Box::from(ListNode { val: 2, next: None })),
        })),
    }));
    assert_eq!(
        eighty_three::delete_duplicates(input),
        Some(Box::from(ListNode {
            val: 1,
            next: Some(Box::from(ListNode { val: 2, next: None })),
        }))
    );
}

#[test]
fn test_largest_rectangle_area() {
    use eighty::eighty_four;
    assert_eq!(
        eighty_four::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]),
        10
    );
    assert_eq!(eighty_four::largest_rectangle_area(vec![2, 1, 0, 2]), 2);
    assert_eq!(eighty_four::largest_rectangle_area(vec![5, 4, 1, 2]), 8);
    assert_eq!(
        eighty_four::largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]),
        6
    );
}
