// 0144. Binary Tree Preorder Traversal
// https://leetcode.cn/problems/binary-tree-preorder-traversal/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Solution::helper(root.as_ref(), &mut res);
        res
    }

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = root {
            // push 根节点
            res.push(node.borrow().val);
            // 访问左子树
            Solution::helper(node.borrow().left.as_ref(), res);
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
    fn test_0144() {
        assert_eq!(
            Solution::preorder_traversal(tree![1, null, 2, 3]),
            vec![1, 2, 3]
        );
        assert_eq!(
            Solution::preorder_traversal(tree![1, 2, 3, 4, 5, 6, 7]),
            vec![1, 2, 4, 5, 3, 6, 7]
        );
        assert_eq!(
            Solution::preorder_traversal(tree![1, 2, 3, 4, 5, null, 6, null, null, 7, 8]),
            vec![1, 2, 4, 5, 7, 8, 3, 6]
        );
    }
}
