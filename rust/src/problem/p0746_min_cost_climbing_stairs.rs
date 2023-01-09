// 0746. Min Cost Climbing Stairs
// https://leetcode.cn/problems/min-cost-climbing-stairs/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        use std::cmp::min;
        let n = cost.len();
        // 可以选择从 0 或 1 开始，初始值全部为 0
        let mut dp = vec![0; n + 1];
        for i in 2..=n {
            // 从 i - 1 阶花费 cost[i - 1] 跨一步到 i
            // 也可以从 i - 2 阶花费 cost[i - 2] 跨两步到 i
            dp[i] = min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2]);
        }
        dp[n]
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0746() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
