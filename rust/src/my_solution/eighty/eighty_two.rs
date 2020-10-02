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
    if !go(m) {
        return m.next.take();
    }
    head
}

fn go(list: &mut Box<ListNode>) -> bool {
    //    println!("{:?}", list);
    if let Some(node) = list.next.as_mut() {
        if list.val == node.val {
            //                println!("list = {:?}", list);
            //                println!("node = {:?}", node);
            list.next = node.next.take();
            //                println!("list = {:?}", list);

            go(list);
            return false;
        } else {
            if !go(node) {
                list.next = node.next.take();
            }
        }
    }
    return true;
}
//作者：albert-ut
//链接：https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list-ii/solution/di-gui-shuang-bai-by-albert-ut/
//来源：力扣（LeetCode）
//著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
