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

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    if lists.len() == 0 {
        return None;
    }
    let mut new_lists = vec![];
    while lists.len() != 1 {
        new_lists = vec![];
        loop {
            match (lists.pop(), lists.pop()) {
                (None, None) => break,
                (Some(t1), None) => new_lists.push(t1),
                (Some(t1), Some(t2)) => new_lists.push(merge_two_lists(t1, t2)),
                _ => {}
            }
        }
        lists = new_lists;
    }
    lists[0].clone()
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (t1, None) => t1,
        (None, t2) => t2,
        (Some(mut t1), Some(mut t2)) => {
            if t1.val < t2.val {
                t1.next = merge_two_lists(t1.next, Some(t2));
                Some(t1)
            } else {
                t2.next = merge_two_lists(Some(t1), t2.next);
                Some(t2)
            }
        }
    }
}
