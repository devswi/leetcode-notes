// 0203. Remove Linked List Elements
// https://leetcode.cn/problems/remove-linked-list-elements/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut store_head = head;
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        while let Some(mut inner) = store_head {
            store_head = inner.next.take();
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
    fn test_0203() {
        assert_eq!(
            Solution::remove_elements(linked![1, 2, 6, 3, 4, 5, 6], 6),
            linked![1, 2, 3, 4, 5]
        );
        assert_eq!(Solution::remove_elements(linked![], 1), linked![]);
        assert_eq!(
            Solution::remove_elements(linked![7, 7, 7, 7, 7], 7),
            linked![]
        );
    }
}
