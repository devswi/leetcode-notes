// 0856. Score of Parentheses
// https://leetcode.cn/problems/score-of-parentheses/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        use std::cmp::max;
        let mut res = vec![0];
        for ch in s.chars() {
            if ch == '(' {
                res.push(0);
            } else if let (Some(v), Some(renew)) = (res.pop(), res.pop()) {
                res.push(renew + max(2 * v, 1));
            }
        }
        res[0]
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0856() {
        assert_eq!(Solution::score_of_parentheses(String::from("()")), 1);
        assert_eq!(Solution::score_of_parentheses(String::from("(())")), 2);
        assert_eq!(Solution::score_of_parentheses(String::from("()()")), 2);
        assert_eq!(Solution::score_of_parentheses(String::from("(()(()))")), 6);
    }
}
