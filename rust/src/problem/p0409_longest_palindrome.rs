// 0409. Longest Palindrome
// https://leetcode.cn/problems/longest-palindrome/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        // 统计字符
        let mut record = HashMap::new();
        for c in s.chars() {
            record.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        let mut res = 0;
        let mut has_odd = false;
        for rec in record.values() {
            // 遍历 s 计算奇偶
            if rec % 2 != 0 {
                has_odd = true;
                res += rec - 1;
            } else {
                res += rec;
            }
        }
        res += if has_odd { 1 } else { 0 };
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0408() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }
}
