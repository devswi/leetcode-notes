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
    pub fn is_valid_bst(_root: Option<Node>) -> bool {
        unimplemented!()
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
    }
}
