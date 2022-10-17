// 0904. Fruit Into Baskets
// https://leetcode.cn/problems/fruit-into-baskets/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut res = vec![];
        for ch in s.chars() {
            if ch == ')' && !res.is_empty() && *res.last().unwrap() == '(' {
                res.pop();
            } else {
                res.push(ch);
            }
        }
        res.len() as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0904() {
        assert_eq!(Solution::min_add_to_make_valid(String::from("())")), 1);
        assert_eq!(Solution::min_add_to_make_valid(String::from("(((")), 3);
    }
}
