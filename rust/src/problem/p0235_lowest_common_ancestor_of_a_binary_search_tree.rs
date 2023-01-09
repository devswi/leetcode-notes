// 0235. Lowest Common Ancestor of a Binary Search Tree
// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Node>,
        p: Option<Node>,
        q: Option<Node>,
    ) -> Option<Node> {
        match (p, q) {
            (Some(p), Some(q)) => Solution::helper(root, p.borrow().val, q.borrow().val),
            _ => None,
        }
    }

    fn helper(root: Option<Node>, p: i32, q: i32) -> Option<Node> {
        if let Some(node) = root {
            if node.borrow().val > p && node.borrow().val > q {
                // 在左子树
                return Solution::helper(node.borrow().left.clone(), p, q);
            }
            if node.borrow().val < p && node.borrow().val < q {
                // 在右子树
                return Solution::helper(node.borrow().right.clone(), p, q);
            }
            return Some(node);
        }
        None
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0235() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5],
                tree![2],
                tree![8]
            ),
            tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5],
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5],
                tree![2],
                tree![4]
            ),
            tree![2, 0, 4, null, null, 3, 5]
        );
    }
}
