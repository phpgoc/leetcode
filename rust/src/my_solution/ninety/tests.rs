use crate::my_solution::ninety;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_subsets_with_dup() {
    let result = ninety::ninety::subsets_with_dup(vec![1, 2, 2]);
    let expect = vec![
        vec![2],
        vec![1],
        vec![1, 2, 2],
        vec![2, 2],
        vec![1, 2],
        vec![],
    ];
    for i in expect {
        assert!(result.contains(&i));
    }
    ninety::ninety::subsets_with_dup(vec![5, 5, 5, 5, 5]);
}

#[test]
fn test_num_decodings() {
    use ninety::ninety_one;
    assert_eq!(ninety_one::num_decodings(String::from("12")), 2);
    assert_eq!(ninety_one::num_decodings(String::from("226")), 3);
    assert_eq!(
        ninety_one::num_decodings(String::from("2611055971756562")),
        4
    );
}

#[test]
fn test_restore_ip_addresses() {
    use ninety::ninety_three;
    let expect = vec![
        String::from("255.255.11.135"),
        String::from("255.255.111.35"),
    ];
    let result = ninety_three::restore_ip_addresses(String::from("25525511135"));
    assert_eq!(result.len(), expect.len());
    for i in result {
        assert!(expect.contains(&i));
    }
    ninety_three::restore_ip_addresses(String::from("010010"));
}

#[test]
fn test_inorder_traversal() {
    use ninety::ninety_four;
    use ninety_four::TreeNode;
    let three = Some(Rc::from(RefCell::from(TreeNode::new(3))));
    let two = Some(Rc::from(RefCell::from(TreeNode {
        val: 2,
        left: three,
        right: None,
    })));
    let one = Some(Rc::from(RefCell::from(TreeNode {
        val: 1,
        left: None,
        right: two,
    })));
    assert_eq!(ninety_four::inorder_traversal(one), vec![1, 3, 2]);
}

#[test]
fn test_is_interleave() {
    use ninety::ninety_seven;
    assert!(ninety_seven::is_interleave(
        String::from("aabcc"),
        String::from("dbbca"),
        String::from("aadbbcbcac"),
    ));
}
