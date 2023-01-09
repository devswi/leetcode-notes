// 0543. Diameter of Binary Tree
// https://leetcode.cn/problems/diameter-of-binary-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Node>) -> i32 {
        let mut res = 0;
        Solution::helper(root, &mut res);
        res
    }

    fn helper(root: Option<Node>, res: &mut i32) -> i32 {
        if let Some(node) = root {
            let left = Solution::helper(node.borrow().left.clone(), res);
            let right = Solution::helper(node.borrow().right.clone(), res);
            // 比较当次最大深度与本次比较
            *res = cmp::max(*res, left + right);
            // 获取二叉树最大深度
            return cmp::max(left, right) + 1;
        }
        0
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0543() {
        assert_eq!(Solution::diameter_of_binary_tree(tree![1, 2, 3, 4, 5]), 3);
        assert_eq!(Solution::diameter_of_binary_tree(tree![1, 2]), 1);
        assert_eq!(
            Solution::diameter_of_binary_tree(tree![
                4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6,
                5, null, 9, null, null, -1, -4, null, null, null, -2
            ]),
            8
        );
    }
}
