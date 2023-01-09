// 0101. Symmetric Tree 对称树
// https://leetcode.cn/problems/symmetric-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn is_symmetric(root: Option<Node>) -> bool {
        if let Some(node) = root {
            return Solution::helper(node.borrow().left.as_ref(), node.borrow().right.as_ref());
        }
        false
    }

    fn helper(left: Option<&Node>, right: Option<&Node>) -> bool {
        return match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                l.borrow().val == r.borrow().val
                    && Solution::helper(l.borrow().left.as_ref(), r.borrow().right.as_ref())
                    && Solution::helper(r.borrow().left.as_ref(), l.borrow().right.as_ref())
            }
            _ => false,
        };
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0101() {
        assert!(Solution::is_symmetric(tree![1, 2, 2, 3, 4, 4, 3]));
        assert!(!Solution::is_symmetric(tree![1, 2, 2, null, 3, null, 3]),);
    }
}
