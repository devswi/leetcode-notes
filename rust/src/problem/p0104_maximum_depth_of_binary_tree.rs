// 0104. Maximum Depth of Binary Tree 二叉树的最大深度
// https://leetcode.cn/problems/maximum-depth-of-binary-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Node>) -> i32 {
        Solution::helper(root, 0)
    }

    fn helper(root: Option<Node>, val: i32) -> i32 {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            return cmp::max(
                Solution::helper(left, val + 1),
                Solution::helper(right, val + 1),
            );
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0104() {
        assert_eq!(Solution::max_depth(tree![3, 9, 20, null, null, 15, 7]), 3);
        assert_eq!(Solution::max_depth(tree![1, null, 2]), 2);
        assert_eq!(Solution::max_depth(tree![]), 0);
        assert_eq!(Solution::max_depth(tree![19]), 1);
    }
}
