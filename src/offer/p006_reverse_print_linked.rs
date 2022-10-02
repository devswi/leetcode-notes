// 剑指 Offer 06. 从尾到头打印链表
// https://leetcode.cn/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;
use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stored_head = head;
        let mut res = VecDeque::new();
        while let Some(inner) = stored_head {
            res.push_front(inner.val);
            stored_head = inner.next;
        }
        Vec::from(res)
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_006() {
        assert_eq!(Solution::reverse_print(linked![1, 3, 2]), vec![2, 3, 1]);
    }
}
