// 0094. Binary Tree Inorder Traversal
// https://leetcode.cn/problems/binary-tree-inorder-traversal/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Solution::helper(root.as_ref(), &mut res);
        res
    }

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = root {
            // 访问左子树
            Solution::helper(node.borrow().left.as_ref(), res);
            // push 根节点
            res.push(node.borrow().val);
            // 访问右子树
            Solution::helper(node.borrow().right.as_ref(), res);
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0094() {
        assert_eq!(
            Solution::inorder_traversal(tree![1, null, 2, 3]),
            vec![1, 3, 2]
        );
        assert_eq!(
            Solution::inorder_traversal(tree![1, 2, 3, 4, 5, 6, 7]),
            vec![4, 2, 5, 1, 6, 3, 7]
        );
        assert_eq!(
            Solution::inorder_traversal(tree![1, 2, 3, 4, 5, null, 6, null, null, 7, 8]),
            vec![4, 2, 7, 5, 8, 1, 3, 6]
        );
    }
}
