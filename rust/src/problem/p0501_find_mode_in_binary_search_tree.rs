// 0501. Find Mode in Binary Search Tree
// https://leetcode.cn/problems/find-mode-in-binary-search-tree/

// 二叉搜索树中序遍历的结果是一个升序队列

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::{cmp::Ordering, rc::Rc};

type Node = Rc<RefCell<TreeNode>>;
pub struct Record {
    current: i32,
    current_times: i32,
    max_times: i32,
}

#[allow(dead_code)]
impl Solution {
    pub fn find_mode(root: Option<Node>) -> Vec<i32> {
        let mut res = vec![];
        match root {
            None => res,
            Some(node) => {
                let val = node.borrow().val;
                let mut record = Record {
                    current: val,
                    current_times: 0,
                    max_times: 0,
                };
                Solution::helper(Some(node), &mut record, &mut res);
                res
            }
        }
    }

    // 中序遍历
    fn helper(root: Option<Node>, record: &mut Record, res: &mut Vec<i32>) {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            // 访问左子树
            Solution::helper(left, record, res);
            let val = node.borrow().val;
            if val == record.current {
                // 相同 current_times + 1
                record.current_times += 1;
            } else {
                // 不相同 current_times 重置为 1
                record.current_times = 1;
            }
            match record.current_times.cmp(&record.max_times) {
                Ordering::Equal => {
                    // 和最大次数相同
                    // push 根节点
                    res.push(val);
                }
                Ordering::Greater => {
                    res.clear();
                    res.push(val);
                    record.max_times = record.current_times;
                }
                _ => {}
            }
            // 保存当前节点值
            record.current = val;
            // 访问右子树
            Solution::helper(right, record, res);
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0501() {
        assert_eq!(Solution::find_mode(tree![1, null, 2, 2]), vec![2]);
        assert_eq!(Solution::find_mode(tree![0]), vec![0]);
        assert_eq!(
            Solution::find_mode(tree![1, 0, 1, 0, 0, 1, 1, 0]),
            vec![0, 1]
        );
    }
}
