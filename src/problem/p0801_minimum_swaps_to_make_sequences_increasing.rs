// 801. Minimum Swaps To Make Sequences Increasing
// https://leetcode.cn/problems/minimum-swaps-to-make-sequences-increasing/

use std::vec;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::cmp::min;
        let len = nums1.len();
        let mut dp = vec![vec![len; 2]; len];
        // dp[i][0] 不交换
        // dp[i][1] 交换
        dp[0][0] = 0;
        dp[0][1] = 1;
        println!("{:?}", dp);
        for i in 1..len {
            if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
                // 顺序满足条件 则
                // 1. i 和 i - 1 都不交换
                dp[i][0] = dp[i - 1][0];
                // 2. i 和 i - 1 都交换
                dp[i][1] = dp[i - 1][1] + 1;
            }
            if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
                // 交叉满足条件，则
                // 1. i 不交换，i - 1 交换
                dp[i][0] = min(dp[i][0], dp[i - 1][1]);
                // 2. i 交换，i - 1 不交换
                dp[i][1] = min(dp[i][1], dp[i - 1][0] + 1);
            }
        }
        min(dp[len - 1][0], dp[len - 1][1]) as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0801() {
        assert_eq!(Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
        assert_eq!(
            Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]),
            1
        );
    }
}
