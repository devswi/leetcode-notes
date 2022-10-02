// 0082. Remove Duplicates from Sorted List II
// https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        let mut stored_head = head;
        let mut pre = i32::MIN;
        while let Some(mut inner) = stored_head {
            stored_head = inner.next.take();
            if stored_head.is_some() && stored_head.as_ref().unwrap().val == inner.val
                || inner.val == pre
            {
                pre = inner.val;
            } else {
                pre = inner.val;
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
    fn test_0082() {
        assert_eq!(
            Solution::remove_duplicates(linked![1, 2, 3, 3, 4, 4, 5]),
            linked![1, 2, 5]
        );
        assert_eq!(
            Solution::remove_duplicates(linked![1, 1, 1, 2, 3]),
            linked![2, 3]
        );
    }
}
