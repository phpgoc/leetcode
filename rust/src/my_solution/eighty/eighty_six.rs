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

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut t = Some(Box::new(ListNode::new(0)));
    t.as_mut().unwrap().next = head;
    let mut head = t;
    let mut tail_head = None;

    let (mut p, mut tail) = (&mut head, &mut tail_head);

    while let Some(node) = &mut p.as_mut().unwrap().next {
        if node.val >= x {
            let t = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            *tail = p.as_mut().unwrap().next.take();
            tail = &mut tail.as_mut().unwrap().next;
            //            println!("t = {:?}", t);
            p.as_mut().unwrap().next = t;
        } else {
            p = &mut p.as_mut().unwrap().next;
            //            println!("p = {:?}", p);
        }
    }
    //    println!("{:?}", tail_head);

    p.as_mut().unwrap().next = tail_head;
    //    println!("{:?}", head);

    head.unwrap().next
}

//作者：icespark
//链接：https://leetcode-cn.com/problems/partition-list/solution/rust-jie-fa-0ms-2mb-shuang-100-by-icespark/
//来源：力扣（LeetCode）
//著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。}
