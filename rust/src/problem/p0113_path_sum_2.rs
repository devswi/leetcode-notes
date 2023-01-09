// 0113. Path Sum II
// https://leetcode.cn/problems/path-sum-ii/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn path_sum(root: Option<Node>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        if root.is_none() {
            return res;
        }
        let mut queue: VecDeque<(i32, Vec<i32>, Option<Node>)> = VecDeque::new();
        queue.push_back((0, Vec::new(), root));
        while !queue.is_empty() {
            if let Some((sum, mut rec, Some(node))) = queue.pop_front() {
                let sum = sum + node.borrow().val;
                rec.push(node.borrow().val);
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if sum == target_sum {
                        res.push(rec);
                    }
                } else {
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back((sum, rec.clone(), Some(left.clone())));
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back((sum, rec.clone(), Some(right.clone())));
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0113() {
        assert_eq!(
            Solution::path_sum(tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1], 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::path_sum(tree![1, 2, 3], 5), res);
    }
}
