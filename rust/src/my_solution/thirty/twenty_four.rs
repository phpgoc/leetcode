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

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => return None,
        Some(mut t) => match t.next {
            None => Some(t),
            Some(mut t2) => {
                t.next = swap_pairs(t2.next);
                t2.next = Some(t);
                Some(t2)
            }
        },
    }
}
