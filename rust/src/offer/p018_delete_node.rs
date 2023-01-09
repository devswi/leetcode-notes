// 剑指 Offer 18. 从尾到头打印链表
// https://leetcode.cn/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        let mut stored_head = head;
        while let Some(mut inner) = stored_head {
            stored_head = inner.next.take();
            if inner.val != val {
                next.as_mut().unwrap().next = Some(inner);
                next = next.unwrap().next.as_mut();
            }
        }
        dummy.unwrap().next
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_018() {
        assert_eq!(
            Solution::delete_node(linked![4, 5, 1, 9], 5),
            linked![4, 1, 9]
        );
        assert_eq!(
            Solution::delete_node(linked![4, 5, 1, 9], 1),
            linked![4, 5, 9]
        );
    }
}
