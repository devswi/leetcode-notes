// 0563. Binary Tree Tilt
// https://leetcode.cn/problems/binary-tree-tilt/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn find_tilt(root: Option<Node>) -> i32 {
        let mut res = 0;
        Solution::helper(root, &mut res);
        res
    }

    fn helper(root: Option<Node>, res: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                let left = Solution::helper(node.borrow().left.clone(), res);
                let right = Solution::helper(node.borrow().right.clone(), res);
                // 坡度之和
                *res += (left - right).abs();
                // 节点只和
                left + right + val
            }
            None => 0,
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0563() {
        assert_eq!(Solution::find_tilt(tree![1, 2, 3]), 1);
        assert_eq!(Solution::find_tilt(tree![4, 2, 9, 3, 5, null, 7]), 15);
        assert_eq!(Solution::find_tilt(tree![21, 7, 14, 1, 1, 2, 2, 3, 3]), 9);
    }
}
