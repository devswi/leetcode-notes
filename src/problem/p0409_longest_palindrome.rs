// 0409. Longest Palindrome
// https://leetcode.cn/problems/longest-palindrome/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(_s: String) -> i32 {
        unimplemented!();
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
