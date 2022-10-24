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
        for num in nums[1..].iter() {
            max_i = max(max_i, *num);
            if max_i <= max_pos {
                pos += 1;
                max_pos = max(*num, max_pos);
            }
        }
        pos + 1
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
    }
}
