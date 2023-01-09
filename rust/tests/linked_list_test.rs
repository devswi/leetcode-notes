use leetcode_in_rust::{data_structure::linked_list::ListNode, linked};

#[test]
fn create_new_linked_node() {
    let node = ListNode::new(1);
    assert_eq!(node.val, 1);
}

#[test]
fn create_new_linked_list_from_vec() {
    let res = linked![1, 2, 3];
    assert_ne!(res, None);
    {
        assert_eq!(res.as_ref().unwrap().val, 1);
    }
    {
        let root = res.unwrap().next;
        assert_ne!(root, None);
        assert_eq!(root.unwrap().val, 2);
    }
}
