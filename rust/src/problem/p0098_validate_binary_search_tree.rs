// 0098. Validate Binary Search Tree
// https://leetcode.cn/problems/validate-binary-search-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_bst(root: Option<Node>) -> bool {
        Solution::helper(root.as_ref(), i64::MIN, i64::MAX)
    }

    fn helper(root: Option<&Node>, lower: i64, upper: i64) -> bool {
        if let Some(node) = root {
            let val = node.borrow().val as i64;
            if val <= lower || val >= upper {
                return false;
            }
            return Solution::helper(node.borrow().left.as_ref(), lower, val)
                && Solution::helper(node.borrow().right.as_ref(), val, upper);
        }
        true
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0098() {
        assert!(Solution::is_valid_bst(tree![2, 1, 3]));
        assert!(!Solution::is_valid_bst(tree![5, 1, 4, null, null, 3, 6]));
        assert!(!Solution::is_valid_bst(tree![2, 2, 2]));
        assert!(!Solution::is_valid_bst(tree![1]));
    }
}
