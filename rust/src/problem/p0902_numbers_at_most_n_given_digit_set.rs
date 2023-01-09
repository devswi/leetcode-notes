// 0902. Numbers At Most N Given Digit Set
// https://leetcode.cn/problems/numbers-at-most-n-given-digit-set/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        // 动态规划
        use std::cmp::Ordering;
        let digits: Vec<char> = digits.iter().map(|s| s.chars().next().unwrap()).collect();
        let n = n.to_string();
        let mut dp = vec![0; n.len() + 1];
        dp[n.len()] = 1;
        for (i, c) in n.chars().rev().enumerate() {
            for &d in digits.iter() {
                match d.cmp(&c) {
                    Ordering::Less => dp[n.len() - i - 1] += digits.len().pow(i as u32),
                    Ordering::Equal => dp[n.len() - i - 1] += dp[n.len() - i],
                    Ordering::Greater => {}
                }
            }
        }
        (dp[0]
            + (1..n.len())
                .map(|i| digits.len().pow(i as u32))
                .sum::<usize>()) as i32
        // 数位 dp 解法，Rust 超时
        // Solution::helper(&n.to_string(), &digits, 0, true, false)
    }

    // i 表示当前填的位数
    // is_limit 前面的数字是否为 n 对应位上的数字，如果 is_limit = true 那么 i 位的数字最多为 s[i]，否则最多为 '9'
    // is_num 前面是否跳过（考虑前导 0 的场景）如果 is_num = true 那么 i 位的数字可以从 0 开始，否则跳过或从 '1' 开始
    fn helper(s: &String, digits: &Vec<String>, i: usize, is_limit: bool, is_num: bool) -> i32 {
        if s.len() == i as usize {
            return if is_num { 1 } else { 0 };
        }
        let mut res = 0;
        if !is_num {
            // 不是数字，选择继续跳过
            res = Solution::helper(s, digits, i + 1, false, false);
        }
        // 依据 is_limit 定义上界
        // is_limit = true 前面填过的数字都对应每一位的数字，那么 i 位最多为 s[i]
        // is_limit = true i 位上的数字最大为 "9"
        let up: char = if is_limit {
            s.chars().nth(i).unwrap()
        } else {
            '9'
        };
        // for 循环执行递归
        for c in digits {
            if c.parse::<u32>().unwrap() > up.to_digit(10).unwrap() {
                break;
            }
            res += Solution::helper(s, digits, i + 1, is_limit && *c == up.to_string(), true)
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0902() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(
                vec![
                    "1".to_string(),
                    "3".to_string(),
                    "5".to_string(),
                    "7".to_string()
                ],
                100
            ),
            20
        );
        assert_eq!(
            Solution::at_most_n_given_digit_set(
                vec!["1".to_string(), "4".to_string(), "9".to_string(),],
                1000000000
            ),
            29523
        );
        assert_eq!(
            Solution::at_most_n_given_digit_set(vec!["7".to_string()], 8),
            1
        );
    }
}
