// 1704. Determine if String Halves Are Alike
// https://leetcode.cn/problems/determine-if-string-halves-are-alike/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mid = s.len() / 2;
        Solution::helper(&s[0..mid]) == Solution::helper(&s[mid..])
    }

    fn helper(s: &str) -> i32 {
        let mut ans = 0;
        for i in s.to_lowercase().chars() {
            match i {
                'a' | 'e' | 'i' | 'o' | 'u' => ans += 1,
                _ => {}
            }
        }
        ans
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1704() {
        assert!(Solution::halves_are_alike("book".to_string()));
        assert!(!Solution::halves_are_alike("textbook".to_string()));
    }
}
