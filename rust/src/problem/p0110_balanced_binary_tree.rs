// 0110. Balanced Binary Tree
// https://leetcode.cn/problems/balanced-binary-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn is_balanced(root: Option<Node>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.as_ref().borrow_mut();
                let left = node.left.as_ref();
                let right = node.right.as_ref();
                if (Solution::helper(left, 0) - Solution::helper(right, 0)).abs() > 1 {
                    return false;
                }
                Solution::is_balanced(node.left.clone())
                    && Solution::is_balanced(node.right.clone())
            }
        }
    }

    fn helper(root: Option<&Node>, height: i32) -> i32 {
        match root {
            None => height,
            Some(node) => max(
                Solution::helper(node.borrow().left.as_ref(), height + 1),
                Solution::helper(node.borrow().right.as_ref(), height + 1),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0110() {
        assert!(Solution::is_balanced(tree![3, 9, 20, null, null, 15, 7]));
        assert!(!Solution::is_balanced(tree![
            1, 2, 2, 3, 3, null, null, 4, 4
        ]));
        assert!(!Solution::is_balanced(tree![
            1, 2, 2, 3, null, null, 3, 4, null, null, 4
        ]));
        assert!(Solution::is_balanced(tree![]));
    }
}
