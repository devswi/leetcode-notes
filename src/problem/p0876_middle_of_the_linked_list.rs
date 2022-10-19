// 0876. Middle of the Linked List
// https://leetcode.cn/problems/middle-of-the-linked-list/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn middle_node(_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0876() {
        assert_eq!(
            Solution::middle_node(linked![1, 2, 3, 4, 5]),
            linked![3, 4, 5]
        );
        assert_eq!(
            Solution::middle_node(linked![1, 2, 3, 4, 5, 6]),
            linked![4, 5, 6]
        );
    }
}
