// 0106. Construct Binary Tree from Preorder and Inorder Traversal
// https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Node> {
        Solution::helper(&inorder, &postorder)
    }

    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Node> {
        // 数组为空，树不存在
        if postorder.is_empty() {
            return None;
        }
        let root_idx = inorder
            .iter()
            .position(|v| v == postorder.last().unwrap())
            .unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: *postorder.last().unwrap(),
            left: Solution::helper(&inorder[0..root_idx], &postorder[0..root_idx]),
            right: Solution::helper(
                &inorder[root_idx + 1..],
                &postorder[root_idx..postorder.len() - 1],
            ),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0106() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3],),
            tree![3, 9, 20, null, null, 15, 7]
        );
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]), tree![-1]);
    }
}
