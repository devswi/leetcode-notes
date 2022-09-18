// 0100. Same Tree
// https://leetcode.cn/problems/same-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // p == q ✅
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                // node val 相同
                let is_val_same = { p_node.borrow().val == q_node.borrow().val };
                // 递归 -> 左子树相同
                let is_left_same = {
                    Solution::is_same_tree(
                        p_node.borrow().left.clone(),
                        q_node.borrow().left.clone(),
                    )
                };
                // 递归 -> 右子树相同
                let is_right_same = {
                    Solution::is_same_tree(
                        p_node.borrow().right.clone(),
                        q_node.borrow().right.clone(),
                    )
                };
                is_val_same && is_left_same && is_right_same
            }
            _ => false,
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0100() {
        assert!(Solution::is_same_tree(tree![1, 2, 3], tree![1, 2, 3]));
        assert!(!Solution::is_same_tree(tree![1, 2], tree![1, null, 2]));
        assert!(!Solution::is_same_tree(tree![1, 2, 1], tree![1, 1, 2]),)
    }
}
