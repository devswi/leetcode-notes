// 0844. Backspace String Compare
// https://leetcode.cn/problems/backspace-string-compare/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let stack = |s: String| -> String {
            let mut res = String::new();
            for c in s.chars() {
                if c == '#' {
                    res.pop();
                } else {
                    res.push(c);
                }
            }
            res
        };
        stack(s) == stack(t)
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0844() {
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
        assert!(Solution::backspace_compare(
            "ab##".to_string(),
            "c#d#".to_string()
        ));
        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));
    }
}
