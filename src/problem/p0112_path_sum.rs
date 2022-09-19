// 0112. Path Sum
// https://leetcode.cn/problems/path-sum/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn has_path_sum(root: Option<Node>, target_sum: i32) -> bool {
        Solution::helper(root, target_sum, 0)
    }

    fn helper(root: Option<Node>, target_sum: i32, sum: i32) -> bool {
        match root {
            Some(node) => {
                let sum = node.borrow().val + sum;
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if left.is_none() && right.is_none() {
                    // 叶子节点，比较 和 与 目标和
                    return sum == target_sum;
                }
                Solution::helper(left, target_sum, sum) || Solution::helper(right, target_sum, sum)
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0112() {
        assert!(Solution::has_path_sum(
            tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
            22
        ));
        assert!(!Solution::has_path_sum(tree![1, 2, 3], 5));
        assert!(!Solution::has_path_sum(tree![], 0));
        assert!(!Solution::has_path_sum(tree![1, 2], 1));
    }
}
