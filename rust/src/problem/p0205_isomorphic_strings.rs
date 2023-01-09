// 0205. Isomorphic Strings
// https://leetcode.cn/problems/isomorphic-strings/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut st = HashMap::new();
        let mut ts = HashMap::new();
        for (ch_s, ch_t) in s.chars().zip(t.chars()) {
            if let Some(c) = st.insert(ch_s, ch_t) {
                if ch_t != c {
                    return false;
                }
            }
            if let Some(c) = ts.insert(ch_t, ch_s) {
                if ch_s != c {
                    return false;
                }
            }
        }
        true
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0205() {
        assert!(Solution::is_isomorphic(
            String::from("egg"),
            String::from("add")
        ));
        assert!(!Solution::is_isomorphic(
            String::from("foo"),
            String::from("bar")
        ));
        assert!(Solution::is_isomorphic(
            String::from("paper"),
            String::from("title")
        ));
        assert!(!Solution::is_isomorphic(
            String::from("badc"),
            String::from("baba")
        ));
    }
}
