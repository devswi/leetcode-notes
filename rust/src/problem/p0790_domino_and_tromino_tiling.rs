// 801. Domino and Tromino Tiling
// https://leetcode.cn/problems/domino-and-tromino-tiling/description/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: usize = 1_000_000_007;
        let n = n as usize;
        let mut dp = vec![vec![0; 2]; n + 1];
        dp[0][0] = 1;
        dp[1][0] = 1;
        for i in 2..=n {
            dp[i][0] = (dp[i - 1][0] + dp[i - 2][0] + 2 * dp[i - 1][1]) % MOD;
            dp[i][1] = (dp[i - 2][0] + dp[i - 1][1]) % MOD;
        }
        dp[n][0] as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0790() {
        assert_eq!(Solution::num_tilings(3), 5);
        assert_eq!(Solution::num_tilings(1), 1);
    }
}
