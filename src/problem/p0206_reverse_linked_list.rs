// 0206. Reverse Linked List
// https://leetcode.cn/problems/remove-linked-list-elements/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unimplemented!();
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0206() {
        assert_eq!(
            Solution::reverse_list(linked![1, 2, 3, 4, 5]),
            linked![5, 4, 3, 2, 1]
        );
        assert_eq!(Solution::reverse_list(linked![1, 2]), linked![2, 1]);
        assert_eq!(Solution::reverse_list(linked![]), linked![]);
    }
}
