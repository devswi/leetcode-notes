// 0020. Valid Parentheses
// https://leetcode.cn/problems/valid-parentheses/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut res = vec![];
        for ch in s.chars() {
            if res.is_empty() {
                res.push(ch);
            } else {
                let last = *res.last().unwrap();
                match (ch, last) {
                    (')', '(') | (']', '[') | ('}', '{') => {
                        res.pop();
                    }
                    _ => {
                        res.push(ch);
                    }
                };
            }
        }
        res.is_empty()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0020() {
        assert!(Solution::is_valid(String::from("()")));
        assert!(Solution::is_valid(String::from("()[]{}")));
        assert!(!Solution::is_valid(String::from("(]")));
    }
}
