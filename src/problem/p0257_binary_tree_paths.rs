// 0257. Binary Tree Paths
// https://leetcode.cn/problems/invert-binary-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn binary_tree_paths(root: Option<Node>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        Solution::helper(root, String::from(""), &mut res);
        res
    }

    fn helper(root: Option<Node>, path: String, res: &mut Vec<String>) {
        if let Some(root) = root {
            if root.borrow().left.is_none() && root.borrow().right.is_none() {
                res.push(format!("{}{}", path, root.borrow().val));
            } else {
                let path = format!("{}{}->", path, root.borrow().val);
                Solution::helper(root.borrow().left.clone(), path.clone(), res);
                Solution::helper(root.borrow().right.clone(), path, res);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0257() {
        assert_eq!(
            Solution::binary_tree_paths(tree![1, 2, 3, null, 5]),
            vec!["1->2->5", "1->3"]
        );
        assert_eq!(
            Solution::binary_tree_paths(tree![1, 2, 3, 4, 5]),
            vec!["1->2->4", "1->2->5", "1->3"]
        );
        assert_eq!(Solution::binary_tree_paths(tree![1]), vec!["1"]);
    }
}
