// 0704. Binary Search
// https://leetcode.cn/problems/binary-search/

use std::cmp::Ordering;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left <= right {
            let mid = (left + right) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Greater => {
                    right = if mid > 0 {
                        mid - 1
                    } else {
                        return -1;
                    };
                }
                Ordering::Equal => {
                    return mid as i32;
                }
            };
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0701() {
        assert_eq!(Solution::binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::binary_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::binary_search(vec![5], 5), 0);
    }
}
