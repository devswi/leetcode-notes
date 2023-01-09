// 0862. Shortest Subarray with Sum at Least K
// https://leetcode.cn/problems/shortest-subarray-with-sum-at-least-k/

#[allow(dead_code)]
pub struct Solution {}

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        // 前缀和
        let mut pre_sum = vec![0; n + 1];

        for i in 0..n {
            pre_sum[i + 1] = pre_sum[i] + nums[i] as i64;
        }

        let mut ans = n + 1;
        let mut queue = VecDeque::new();

        for i in 0..=n {
            while let Some(&j) = queue.back() {
                if pre_sum[i] <= pre_sum[j] {
                    queue.pop_back();
                } else {
                    break;
                }
            }

            while let Some(&j) = queue.front() {
                if pre_sum[i] >= pre_sum[j] + k as i64 {
                    ans = ans.min(i - j);
                    queue.pop_front();
                } else {
                    break;
                }
            }

            queue.push_back(i)
        }

        if ans == n + 1 {
            -1
        } else {
            ans as i32
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0862() {
        assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
        assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1);
        assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3);
    }
}
