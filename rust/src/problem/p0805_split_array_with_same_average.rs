// 805. Split Array With Same Average
// https://leetcode.cn/problems/split-array-with-same-average/

use std::collections::HashSet;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn split_array_with_same_average(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let m = n / 2;
        if n == 1 {
            return false;
        }
        let sum = nums.iter().map(|&x| x as i32).sum::<i32>();
        let nums = nums
            .iter()
            .map(|&x| x * (n as i32) - sum)
            .collect::<Vec<i32>>();
        let mut left = HashSet::new();
        for i in 1..(1 << m) {
            let mut tot = 0;
            (0..m).for_each(|j| {
                if (i & (1 << j)) != 0 {
                    tot += nums[j];
                }
            });
            if tot == 0 {
                return true;
            }
            left.insert(tot);
        }
        let mut rsum = 0;
        (m..n).for_each(|i| rsum += nums[i]);
        for i in 1..(1 << (n - m)) {
            let mut tot = 0;
            (m..n).for_each(|j| {
                if (i & (1 << (j - m))) != 0 {
                    tot += nums[j];
                }
            });
            if tot == 0 || (rsum != tot && left.contains(&-tot)) {
                return true;
            }
        }
        false
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0805() {
        assert!(Solution::split_array_with_same_average(vec![
            1, 2, 3, 4, 5, 6, 7, 8
        ]),);
        assert!(!Solution::split_array_with_same_average(vec![3, 1]),);
        assert!(!Solution::split_array_with_same_average(vec![
            2, 12, 18, 16, 19, 3
        ]));
        assert!(Solution::split_array_with_same_average(vec![0, 0, 3, 9, 8]));
    }
}
