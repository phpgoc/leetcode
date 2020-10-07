// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    if root.is_some() {
        recursive(root.unwrap(), &mut res);
    }
    res
}

fn recursive(mut root: Rc<RefCell<TreeNode>>, res: &mut Vec<i32>) {
    let val = root.borrow().val;
    let mut node = root.borrow_mut().left.take();

    if node.is_some() {
        recursive(node.unwrap(), res);
    }
    res.push(val);
    node = root.borrow_mut().right.take();

    if node.is_some() {
        recursive(node.unwrap(), res);
    }
}
