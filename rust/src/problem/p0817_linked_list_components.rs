// 0817. Linked List Components
// https://leetcode.cn/problems/linked-list-components/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::linked_list::ListNode;

#[allow(dead_code)]
impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        // 遍历链表
        let mut res = 0;
        let mut pre = false;
        let mut head = head;
        while let Some(node) = head {
            head = node.next;
            let exist = nums.contains(&node.val);
            if !pre && exist {
                res += 1;
            }
            pre = exist;
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0817() {
        assert_eq!(
            Solution::num_components(linked![0, 1, 2, 3], vec![0, 1, 3]),
            2
        );
        assert_eq!(
            Solution::num_components(linked![0, 1, 2, 3, 4], vec![0, 3, 1, 4]),
            2
        );
        assert_eq!(
            Solution::num_components(linked![0, 1, 2, 3, 4], vec![0, 3, 1, 2, 4]),
            1
        );
        assert_eq!(Solution::num_components(linked![3, 4, 0, 2, 1], vec![4]), 1);
    }
}
