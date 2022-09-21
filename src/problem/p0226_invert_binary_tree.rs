// 0226. Invert Binary Tree
// https://leetcode.cn/problems/invert-binary-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn invert_tree(root: Option<Node>) -> Option<Node> {
        if let Some(node) = root.clone() {
            Solution::invert_tree(node.borrow().left.clone());
            Solution::invert_tree(node.borrow().right.clone());
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0226() {
        assert_eq!(
            Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
            tree![4, 7, 2, 9, 6, 3, 1]
        );
        assert_eq!(Solution::invert_tree(tree![2, 1, 3]), tree![2, 3, 1]);
    }
}
