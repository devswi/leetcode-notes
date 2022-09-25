// 0102. Binary Tree Level Order Traversal 层序遍历
// https://leetcode.cn/problems/binary-tree-level-order-traversal/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Node>) -> Vec<Vec<i32>> {
        // 遍历队列
        let mut queue: VecDeque<Node> = VecDeque::new();
        let mut res: Vec<Vec<i32>> = vec![];

        if let Some(node) = root {
            // 初始，push 根节点
            queue.push_back(node);
            let mut size: usize = queue.len();
            let mut tmp: Vec<i32> = Vec::new();
            while !queue.is_empty() {
                // pop 队首
                if let Some(node) = queue.pop_front() {
                    size -= 1;
                    tmp.push(node.borrow().val);

                    // push 左右子树
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right)
                    }

                    if size == 0 {
                        // size 为 0 一层访问结束，保存一层的结果
                        res.push(tmp);
                        size = queue.len();
                        tmp = Vec::new();
                    }
                }
            }
            // 终止条件 => queue 为空
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0102() {
        assert_eq!(
            Solution::level_order(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
        assert_eq!(Solution::level_order(tree![1]), vec![vec![1]]);
    }
}
