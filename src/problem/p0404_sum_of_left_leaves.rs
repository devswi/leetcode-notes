// 0404. Sum of Left Leaves
// https://leetcode.cn/problems/sum-of-left-leaves/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Node>) -> i32 {
        let mut sum = 0;
        Solution::helper(root, &mut sum);
        sum
    }

    fn helper(root: Option<Node>, sum: &mut i32) {
        if let Some(node) = root {
            if let Some(left) = node.borrow().left.clone() {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    *sum += left.borrow().val;
                }
            }
            Solution::helper(node.borrow().left.clone(), sum);
            Solution::helper(node.borrow().right.clone(), sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0404() {
        assert_eq!(
            Solution::sum_of_left_leaves(tree![3, 9, 20, null, null, 15, 7]),
            24
        );
        assert_eq!(Solution::sum_of_left_leaves(tree![1]), 0);
    }
}
