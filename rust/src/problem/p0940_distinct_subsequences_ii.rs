// 0940. Distinct Subsequences II
// https://leetcode.cn/problems/distinct-subsequences-ii/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let n = s.len();
        const MOD: usize = 1_000_000_007;
        let mut dp = vec![0; n];
        dp[0] = 1;
        for i in 1..n {
            // dp[i] = 2 * dp[i] + 1
            // i - 1 的各种情况加上 i 字符
            dp[i] = (2 * dp[i - 1] + 1) % MOD;
            for j in (0..=i - 1).rev() {
                if let (Some(char_i), Some(char_j)) = (s.chars().nth(i), s.chars().nth(j)) {
                    if char_i == char_j {
                        // 包含重复
                        dp[i] = if j == 0 {
                            dp[i] - 1
                        } else {
                            (dp[i] - dp[j - 1] - 1 + MOD) % MOD
                        };
                        break;
                    }
                }
            }
        }
        dp[n - 1] as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0940() {
        assert_eq!(Solution::distinct_subseq_ii(String::from("abc")), 7);
        assert_eq!(Solution::distinct_subseq_ii(String::from("aba")), 6);
        assert_eq!(Solution::distinct_subseq_ii(String::from("aaa")), 3);
        assert_eq!(Solution::distinct_subseq_ii(String::from("lee")), 5);
    }
}
