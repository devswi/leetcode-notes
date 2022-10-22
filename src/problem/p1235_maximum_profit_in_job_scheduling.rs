// 1235. Maximum Profit in Job Scheduling
// https://leetcode.cn/problems/maximum-profit-in-job-scheduling/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        use std::cmp::max;
        let len = start_time.len();
        // 整合工作
        let mut jobs = vec![(0, 0, 0); len];
        for i in 0..len {
            jobs[i] = (start_time[i], end_time[i], profit[i]);
        }
        // 按结束时间排序
        jobs.sort_by(|&a, &b| a.1.cmp(&b.1));
        // 创建 dp 数组
        let mut dp = vec![0; len + 1];
        for i in 1..=len {
            let k = {
                let mut left = 0;
                let mut right = i - 1;
                let target = jobs[i - 1].0;
                while left < right {
                    let mid = left + (right - left) / 2;
                    // 结束时间 <= 开始时间
                    if jobs[mid].1 <= target {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
                left
            };
            dp[i] = max(dp[i - 1], dp[k] + jobs[i - 1].2);
        }
        dp[len]
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1235() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );
        assert_eq!(
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6
        );
    }
}
