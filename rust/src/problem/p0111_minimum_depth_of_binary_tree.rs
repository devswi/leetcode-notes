// 0111. Minimum Depth of Binary Tree
// https://leetcode.cn/problems/minimum-depth-of-binary-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn min_depth(root: Option<Node>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let root = node.as_ref().borrow_mut();
                match (root.left.as_ref(), root.right.as_ref()) {
                    (Some(l), None) => 1 + Solution::min_depth(Some(l.clone())),
                    (None, Some(r)) => 1 + Solution::min_depth(Some(r.clone())),
                    (Some(l), Some(r)) => {
                        1 + cmp::min(
                            Solution::min_depth(Some(l.clone())),
                            Solution::min_depth(Some(r.clone())),
                        )
                    }
                    _ => 1,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0111() {
        assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
        assert_eq!(
            Solution::min_depth(tree![2, null, 3, null, 4, null, 5, null, 6]),
            5
        );
    }
}
