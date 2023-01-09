// 0637. Average of Levels in Binary Tree
// https://leetcode.cn/problems/average-of-levels-in-binary-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn average_of_levels(root: Option<Node>) -> Vec<f64> {
        let mut queue: VecDeque<Node> = VecDeque::new();
        let mut res: Vec<f64> = vec![];
        if let Some(node) = root {
            queue.push_back(node);
            let mut size: usize = queue.len();
            let mut old_size: usize = queue.len();
            let mut sum: i64 = 0;
            while !queue.is_empty() {
                if let Some(node) = queue.pop_front() {
                    size -= 1;
                    sum += node.borrow().val as i64;

                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right);
                    }

                    if size == 0 {
                        res.push((sum as f64) / (old_size as f64));
                        sum = 0;
                        size = queue.len();
                        old_size = queue.len();
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
    fn test_0637() {
        assert_eq!(
            Solution::average_of_levels(tree![3, 9, 20, null, null, 15, 7]),
            vec![3.0, 14.5, 11.0]
        );
        assert_eq!(
            Solution::average_of_levels(tree![3, 9, 20, 15, 7]),
            vec![3.0, 14.5, 11.0]
        );
        assert_eq!(
            Solution::average_of_levels(tree![2147483647, 2147483647, 2147483647]),
            vec![2147483647.0, 2147483647.0]
        );
    }
}
