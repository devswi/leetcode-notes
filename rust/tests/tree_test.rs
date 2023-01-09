use leetcode_in_rust::{data_structure::tree::TreeNode, tree};

#[test]
fn create_new_tree() {
    let tree = TreeNode::new(1);
    assert_eq!(tree.val, 1);
}

#[test]
fn create_new_tree_from_vec() {
    let res = tree![1, null, 2, 3];
    assert_ne!(res, None);

    let res = res.unwrap();

    let root = res.borrow();
    assert_eq!(root.val, 1);

    {
        let res = res.borrow();
        assert_eq!(res.left, None);
    }

    {
        let res = res.borrow();
        assert_ne!(res.right, None);
        let right_children = res.right.clone().unwrap();
        assert_eq!(right_children.borrow().val, 2);
        assert_ne!(right_children.borrow().left, None);
    }
}
