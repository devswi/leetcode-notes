// 0503. Minimum Absolute Difference in BST
// https://leetcode.cn/problems/minimum-absolute-difference-in-bst/

// 二叉搜索树中序遍历的结果是一个升序队列

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::rc::Rc;
use std::{cell::RefCell, cmp};

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn get_minimum_difference(root: Option<Node>) -> i32 {
        let mut res = i32::MAX;
        let mut pre: Option<i32> = None;
        match root {
            None => res,
            Some(node) => {
                Solution::helper(Some(node), &mut pre, &mut res);
                res
            }
        }
    }

    // 中序遍历
    fn helper(root: Option<Node>, pre: &mut Option<i32>, res: &mut i32) {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            // 访问左子树
            Solution::helper(left, pre, res);
            let val = node.borrow().val;
            if let Some(pre) = pre {
                *res = cmp::min(val - *pre, *res);
            }
            // 更新 pre
            *pre = Some(val);
            let right = node.borrow().right.clone();
            // 访问右子树
            Solution::helper(right, pre, res);
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0530() {
        assert_eq!(Solution::get_minimum_difference(tree![4, 2, 6, 1, 3]), 1);
        assert_eq!(
            Solution::get_minimum_difference(tree![1, 0, 48, null, null, 12, 49]),
            1
        );
        assert_eq!(
            Solution::get_minimum_difference(tree![543, 384, 652, null, 445, null, 699]),
            47
        )
    }
}
