// 剑指 Offer 07. 重建二叉树
// https://leetcode.cn/problems/zhong-jian-er-cha-shu-lcof/
// 同主站 p0105

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Node> {
        Solution::helper(&preorder, &inorder)
    }

    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Node> {
        if preorder.is_empty() {
            return None;
        }
        let root = preorder.first().unwrap();
        let pos = inorder.iter().position(|&x| &x == root).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: *root,
            left: Solution::helper(&preorder[1..=pos], &inorder[..pos]),
            right: Solution::helper(&preorder[pos + 1..], &inorder[pos + 1..]),
        })))
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_007() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            tree![3, 9, 20, null, null, 15, 7]
        );
    }
}
