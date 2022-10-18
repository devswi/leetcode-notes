// 0021. Merge Two Sorted Lists
// https://leetcode.cn/problems/merge-two-sorted-lists/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy;
        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
            let l = if l1.val < l2.val {
                &mut list1
            } else {
                &mut list2
            };
            p.next = l.take();
            p = p.next.as_mut().unwrap();
            *l = p.next.take();
        }
        p.next = list1.or(list2);
        dummy.next
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0021() {
        assert_eq!(
            Solution::merge_two_lists(linked![1, 2, 4], linked![1, 3, 4]),
            linked![1, 1, 2, 3, 4, 4]
        );
        assert_eq!(Solution::merge_two_lists(linked![], linked![]), linked![]);
        assert_eq!(Solution::merge_two_lists(linked![], linked![0]), linked![0]);
    }
}
