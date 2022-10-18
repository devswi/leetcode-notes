// 0021. Reverse Linked List
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
        unimplemented!();
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
