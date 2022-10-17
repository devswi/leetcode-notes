// 0392. Is Subsequence
// https://leetcode.cn/problems/is-subsequence/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let s_vec: Vec<char> = s.chars().collect();
        let mut i = 0;
        for ch_t in t.chars() {
            if let Some(ch) = s_vec.get(i) {
                if *ch == ch_t {
                    i += 1;
                }
            }
        }
        s.len() == i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0392() {
        assert!(Solution::is_subsequence(
            String::from("abc"),
            String::from("ahbgdc")
        ));
        assert!(!Solution::is_subsequence(
            String::from("axc"),
            String::from("ahbgdc")
        ));
        assert!(Solution::is_subsequence(
            String::from(""),
            String::from("ahbgdc")
        ));
        assert!(!Solution::is_subsequence(
            String::from("sd"),
            String::from("")
        ));
        assert!(Solution::is_subsequence(
            String::from("b"),
            String::from("abc")
        ));
    }
}
