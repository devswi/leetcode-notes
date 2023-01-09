// 0002. Add Two Numbers
// https://leetcode.cn/problems/add-two-numbers/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut index = 0; // 进位
        let mut dummy: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut l1 = l1;
        let mut l2 = l2;
        let mut next = dummy.as_mut();
        while l1.is_some() || l2.is_some() {
            let mut val = index;
            if let Some(mut node) = l1 {
                val += node.val;
                l1 = node.next.take();
            }
            if let Some(mut node) = l2 {
                val += node.val;
                l2 = node.next.take();
            }
            index = val / 10;
            next.as_mut().unwrap().next = Some(Box::new(ListNode::new(val % 10)));
            next = next.unwrap().next.as_mut();
        }
        if index != 0 {
            next.as_mut().unwrap().next = Some(Box::new(ListNode::new(index)));
        }
        dummy.unwrap().next
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0002() {
        assert_eq!(
            Solution::add_two_numbers(linked![2, 4, 3], linked![5, 6, 4]),
            linked![7, 0, 8]
        );
        assert_eq!(
            Solution::add_two_numbers(linked![0], linked![0]),
            linked![0]
        );
        assert_eq!(
            Solution::add_two_numbers(linked![9, 9, 9, 9, 9, 9, 9], linked![9, 9, 9, 9]),
            linked![8, 9, 9, 9, 0, 0, 0, 1]
        );
        assert_eq!(
            Solution::add_two_numbers(linked![2, 4, 9], linked![5, 6, 4, 9]),
            linked![7, 0, 4, 0, 1]
        );
    }
}
