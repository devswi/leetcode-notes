// 0513. Find Bottom Left Tree Value
// https://leetcode.cn/problems/find-bottom-left-tree-value/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn find_bottom_left_value(root: Option<Node>) -> i32 {
        let mut res = 0;
        let mut queue: VecDeque<Node> = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
            while !queue.is_empty() {
                let size = queue.len();
                for i in 0..size {
                    if let Some(node) = queue.pop_front() {
                        if i == 0 {
                            res = node.borrow().val;
                        }
                        if let Some(left) = node.borrow().left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = node.borrow().right.clone() {
                            queue.push_back(right);
                        }
                    }
                }
            }
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0513() {
        assert_eq!(Solution::find_bottom_left_value(tree![2, 1, 3]), 1);
        assert_eq!(
            Solution::find_bottom_left_value(tree![1, 2, 3, 4, null, 5, 6, null, null, 7]),
            7
        );
        assert_eq!(
            Solution::find_bottom_left_value(tree![1, 2, 3, null, 5, 6, 7]),
            5
        );
    }
}
