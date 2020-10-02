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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut head = head;
    let m = head.as_mut().unwrap();
    go(m);
    head
}

fn go(list: &mut Box<ListNode>) {
    //    println!("{:?}", list);
    if let Some(node) = list.next.as_mut() {
        if list.val == node.val {
            list.next = node.next.take();
            go(list);
        } else {
            go(node);
        }
    }
}
