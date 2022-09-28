// 0105. Construct Binary Tree from Preorder and Inorder Traversal
// https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Node> {
        Solution::helper(&preorder[..], &inorder[..])
    }

    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Node> {
        if preorder.is_empty() {
            return None;
        }
        // 前序遍历的首个元素为根节点
        let root_idx = inorder
            .iter()
            .position(|v| v == preorder.first().unwrap())
            .unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: *preorder.first().unwrap(),
            left: Solution::helper(&preorder[1..root_idx + 1], &inorder[..root_idx]),
            right: Solution::helper(&preorder[root_idx + 1..], &inorder[root_idx + 1..]),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0105() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            tree![3, 9, 20, null, null, 15, 7]
        );
        // assert_eq!(Solution::build_tree(vec![-1], vec![-1]), tree![-1]);
    }
}
