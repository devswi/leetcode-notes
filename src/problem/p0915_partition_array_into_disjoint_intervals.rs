// 0915. Partition Array into Disjoint Intervals
// https://leetcode.cn/problems/partition-array-into-disjoint-intervals/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        if nums.is_empty() {
            return -1;
        }
        let mut pos = 0;
        // 0~pos 的最大值
        let mut max_pos = nums[0];
        // 0~i 中的最大值
        let mut max_i = nums[0];
        (1..nums.len()).for_each(|i| {
            max_i = max(max_i, nums[i]);
            if max_pos > nums[i] {
                max_pos = max_i;
                pos = i;
            }
        });
        (pos + 1) as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0915() {
        assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
        assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
        assert_eq!(Solution::partition_disjoint(vec![1, 1]), 1);
    }
}
