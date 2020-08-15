use std::borrow::BorrowMut;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut n = n;
    remove_nth_from_end_recursion(head, n.borrow_mut())
}

fn remove_nth_from_end_recursion(
    head: Option<Box<ListNode>>,
    n: &mut i32,
) -> Option<Box<ListNode>> {
    let mut linlist: Option<Box<ListNode>> = head;
    match linlist.take() {
        None => None,
        Some(mut x) => {
            let next = remove_nth_from_end_recursion(x.next.take(), n);
            *n -= 1;
            if *n == 0 {
                return next;
            } else {
                x.next = next;
                return Some(x);
            }
        }
    }
}
