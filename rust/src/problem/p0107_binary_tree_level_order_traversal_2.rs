// 0107. Binary Tree Level Order Traversal II 层序遍历
// https://leetcode.cn/problems/binary-tree-level-order-traversal-ii/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn level_order_bottom(root: Option<Node>) -> Vec<Vec<i32>> {
        // 遍历队列
        let mut queue: VecDeque<Node> = VecDeque::new();
        let mut res: VecDeque<Vec<i32>> = VecDeque::new();

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
                        res.push_front(tmp);
                        size = queue.len();
                        tmp = Vec::new();
                    }
                }
            }
            // 终止条件 => queue 为空
        }
        Vec::from(res)
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0107() {
        assert_eq!(
            Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![15, 7], vec![9, 20], vec![3]]
        );
        assert_eq!(Solution::level_order_bottom(tree![1]), vec![vec![1]]);
    }
}
