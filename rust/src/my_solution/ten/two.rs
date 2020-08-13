use std::collections::LinkedList;

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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut c1 = &l1;
    let mut c2 = &l2;
    let mut count: i32 = 0;
    let mut head = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut head;

    loop {
        if c1.is_none() && c2.is_none() {
            if count == 1 {
                tail.next = Some(Box::new(ListNode { val: 1, next: None }));
                tail = tail.next.as_mut().unwrap();
            }
            break;
        }
        let mut add = count;
        if let Some(t) = c1 {
            add += t.val;
            c1 = &t.next;
        };
        if let Some(t) = c2 {
            add += t.val;
            c2 = &t.next;
        };
        let added_val = (add) % 10;
        count = (add) / 10;
        tail.next = Some(Box::new(ListNode {
            val: added_val,
            next: None,
        }));
        tail = tail.next.as_mut().unwrap();
    }
    head.next
}
