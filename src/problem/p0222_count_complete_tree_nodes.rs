// 0222. Count Complete Tree Nodes
// https://leetcode.cn/problems/count-complete-tree-nodes/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn count_nodes(root: Option<Node>) -> i32 {
        if let Some(node) = root {
            let mut left_depth = 0;
            let mut left = node.borrow().left.clone();
            while left.is_some() {
                left_depth += 1;
                left = left.unwrap().borrow().left.clone();
            }

            let mut right_depth = 0;
            let mut right = node.borrow().right.clone();
            while right.is_some() {
                right_depth += 1;
                right = right.unwrap().borrow().right.clone();
            }

            if left_depth == right_depth {
                return (2 << left_depth) - 1;
            }
            return Solution::count_nodes(node.borrow().left.clone())
                + Solution::count_nodes(node.borrow().right.clone())
                + 1; // 根节点
        }
        0
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0222() {
        assert_eq!(Solution::count_nodes(tree![1, 2, 3, 4, 5, 6]), 6);
        assert_eq!(Solution::count_nodes(tree![]), 0);
        assert_eq!(Solution::count_nodes(tree![1]), 1);
    }
}
