// 0515. Find Largest Value in Each Tree Row
// https://leetcode.cn/problems/find-largest-value-in-each-tree-row/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn largest_values(root: Option<Node>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut queue: VecDeque<Node> = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
            let mut size = queue.len();
            let mut val = i32::MIN;
            while !queue.is_empty() {
                if let Some(node) = queue.pop_front() {
                    size -= 1;
                    val = cmp::max(val, node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right);
                    }
                    if size == 0 {
                        res.push(val);
                        size = queue.len();
                        val = i32::MIN;
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
    fn test_0515() {
        assert_eq!(
            Solution::largest_values(tree![1, 3, 2, 5, 3, null, 9]),
            vec![1, 3, 9]
        );
        assert_eq!(Solution::largest_values(tree![1, 2, 3]), vec![1, 3]);
        assert_eq!(Solution::largest_values(tree![0, -1]), vec![0, -1]);
    }
}
