use std::borrow::BorrowMut;

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

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    if k == 0 || head.is_none() {
        return head;
    }

    // 找到链表的长度，rust无法构建一个链表环
    let mut ref_node = &mut head;
    let mut len = 0;
    while let Some(node) = ref_node {
        len += 1;
        ref_node = &mut node.next;
    }

    let i = len - (k % len); // 注意要先取余
    if len == 1 || i == len {
        return head;
    }

    // 找到打断的位置
    ref_node = &mut head;
    for _ in 0..i {
        if let Some(node) = ref_node {
            ref_node = &mut node.next;
        }
    }

    // 把链表打断为两端
    let mut remain = ref_node.take();
    let mut ref_node = &mut remain;
    if head.is_none() {
        return remain;
    }

    // 重新拼接两段链表
    while let Some(node) = ref_node {
        if node.next.is_none() {
            // 将前面一段链表接到后面的一段链表里
            node.next.replace(head.unwrap());
            break;
        }
        ref_node = &mut node.next;
    }

    remain
}

//作者：kamuel
//链接：https://leetcode-cn.com/problems/rotate-list/solution/rustwu-fa-ba-lian-biao-shou-wei-xiang-jie-gou-chen/
//来源：力扣（LeetCode）
//著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
