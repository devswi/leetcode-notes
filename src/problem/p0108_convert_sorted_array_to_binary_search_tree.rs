// 0108. Convert Sorted Array to Binary Search Tree
// https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    // 二分数组
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Node> {
        Solution::helper(&nums, 0, nums.len())
    }

    fn helper(nums: &Vec<i32>, left: usize, right: usize) -> Option<Node> {
        if left == right {
            return None;
        }
        let mid = (left + right) / 2;
        let mut root = TreeNode::new(nums[mid]);
        // 左右子树的构造都是 左闭又开
        root.left = Solution::helper(nums, left, mid);
        root.right = Solution::helper(nums, mid + 1, right);
        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0104() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            tree![0, -3, 9, -10, null, 5]
        );
        assert_eq!(Solution::sorted_array_to_bst(vec![1, 3]), tree![3, 1]);
    }
}
