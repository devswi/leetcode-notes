// 0199. Binary Tree Right Side View
// https://leetcode.cn/problems/binary-tree-right-side-view/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn right_side_view(root: Option<Node>) -> Vec<i32> {
        // 遍历队列
        let mut queue: VecDeque<Node> = VecDeque::new();
        let mut res: Vec<i32> = vec![];
        if let Some(node) = root {
            queue.push_back(node);
            let mut size: usize = queue.len();
            while !queue.is_empty() {
                if let Some(node) = queue.pop_front() {
                    size -= 1;
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right);
                    }
                    if size == 0 {
                        res.push(node.borrow().val);
                        size = queue.len();
                    }
                }
            }
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0199() {
        assert_eq!(
            Solution::right_side_view(tree![1, 2, 3, null, 5, null, 4]),
            vec![1, 3, 4]
        );
        assert_eq!(Solution::right_side_view(tree![1, null, 3]), vec![1, 3]);
        assert_eq!(Solution::right_side_view(tree![]), vec![]);
    }
}
