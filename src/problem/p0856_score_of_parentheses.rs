// 0856. Score of Parentheses
// https://leetcode.cn/problems/score-of-parentheses/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0870() {
        assert_eq!(Solution::score_of_parentheses(String::from("()")), 1);
        assert_eq!(Solution::score_of_parentheses(String::from("(())")), 2);
        assert_eq!(Solution::score_of_parentheses(String::from("()()")), 2);
        assert_eq!(Solution::score_of_parentheses(String::from("(()(()))")), 6);
    }
}
