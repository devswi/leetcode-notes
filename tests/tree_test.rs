use leetcode_in_rust::data_structure::tree::TreeNode;

#[test]
fn create_new_tree() {
    let tree = TreeNode::new(1);
    assert_eq!(tree.val, 1);
}
