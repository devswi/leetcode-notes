// 0700. Search in a Binary Search Tree
// https://leetcode.cn/problems/search-in-a-binary-search-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn search_bst(root: Option<Node>, val: i32) -> Option<Node> {
        if root.is_none() {
            return root;
        }
        let root = root.unwrap();
        let current = root.borrow().val;
        match val.cmp(&current) {
            Ordering::Equal => Some(root),
            Ordering::Less => {
                // 左子树
                let left = root.borrow().left.clone();
                Solution::search_bst(left, val)
            }
            Ordering::Greater => {
                // 右子树
                let right = root.borrow().right.clone();
                Solution::search_bst(right, val)
            }
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0700() {
        assert_eq!(
            Solution::search_bst(tree![4, 2, 7, 1, 3], 2),
            tree![2, 1, 3]
        );
        assert_eq!(Solution::search_bst(tree![4, 2, 7, 1, 3], 5), None);
    }
}
