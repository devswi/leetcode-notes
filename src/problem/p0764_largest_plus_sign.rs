// 0764. Largest Plus Sign
// https://leetcode.cn/problems/largest-plus-sign/

use std::iter::zip;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![n as i32; n]; n];
        for m in mines {
            dp[m[0] as usize][m[1] as usize] = 0;
        }
        for i in 0..n {
            let (mut top, mut right, mut bottom, mut left) = (0, 0, 0, 0);
            for (j, k) in zip(0..n, (0..n).rev()) {
                top = if dp[j][i] == 0 { 0 } else { top + 1 };
                right = if dp[i][k] == 0 { 0 } else { right + 1 };
                bottom = if dp[k][i] == 0 { 0 } else { bottom + 1 };
                left = if dp[i][j] == 0 { 0 } else { left + 1 };
                dp[j][i] = dp[j][i].min(top);
                dp[i][k] = dp[i][k].min(right);
                dp[k][i] = dp[k][i].min(bottom);
                dp[i][j] = dp[i][j].min(left);
            }
        }
        let mut ans = 0;
        zip(0..n, 0..n).for_each(|(i, j)| ans = ans.max(dp[i][j]));
        ans
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0764() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
        assert_eq!(Solution::order_of_largest_plus_sign(1, vec![vec![0, 0]]), 0);
    }
}
