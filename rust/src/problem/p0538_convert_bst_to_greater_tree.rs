// 0538. Convert BST to Greater Tree
// https://leetcode.cn/problems/convert-bst-to-greater-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn convert_bst(root: Option<Node>) -> Option<Node> {
        let mut sum = 0;
        Solution::helper(root.clone(), &mut sum);
        root
    }

    fn helper(root: Option<Node>, sum: &mut i32) {
        if let Some(node) = root {
            Solution::helper(node.borrow().right.clone(), sum);
            *sum += node.borrow().val;
            node.borrow_mut().val = *sum;
            Solution::helper(node.borrow().left.clone(), sum);
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0538() {
        assert_eq!(
            Solution::convert_bst(tree![
                4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
            ]),
            tree![30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8]
        );
        assert_eq!(Solution::convert_bst(tree![0, null, 1]), tree![1, null, 1]);
    }
}
