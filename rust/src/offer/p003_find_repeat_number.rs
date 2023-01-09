// 剑指 Offer 03. 数组中重复的数字
// https://leetcode.cn/problems/shu-zu-zhong-zhong-fu-de-shu-zi-lcof/?favorite=xb9nqhhg

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        use std::collections::{hash_map, HashMap};
        let mut record: HashMap<i32, i32> = HashMap::new();
        for val in nums {
            if let hash_map::Entry::Vacant(e) = record.entry(val) {
                e.insert(0);
            } else {
                return val;
            }
        }
        -1
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_003() {
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]), 2);
    }
}
