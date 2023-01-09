// 0816. Ambiguous Coordinates
// https://leetcode.cn/problems/ambiguous-coordinates/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let len = s.len();
        let mut res = vec![];

        for i in 1..len - 1 {
            for first in Solution::get_all_result(&s[1..i]) {
                for second in Solution::get_all_result(&s[i..len - 1]) {
                    res.push(format!("({}, {})", first, second));
                }
            }
        }

        res
    }

    fn get_all_result(s: &str) -> Vec<String> {
        let mut res = vec![];
        if s.is_empty() {
            return res;
        }
        if !s.starts_with('0') || s == "0" {
            res.push(s.to_owned());
        }
        for i in 1..s.len() {
            // 整数部分，为 0 或 不以 0 开头
            let integer = &s[..i];
            // 小数部分不以 0 结尾
            let decimals = &s[i..];
            if (integer == "0" || !integer.starts_with('0')) && !decimals.ends_with('0') {
                res.push(format!("{}.{}", &s[..i], &s[i..]));
            }
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0816() {
        assert_eq!(
            Solution::ambiguous_coordinates("(123)".to_string()),
            vec![
                "(1, 23)".to_string(),
                "(1, 2.3)".to_string(),
                "(12, 3)".to_string(),
                "(1.2, 3)".to_string(),
            ]
        );
        assert_eq!(
            Solution::ambiguous_coordinates("(0123)".to_string()),
            vec![
                "(0, 123)".to_string(),
                "(0, 1.23)".to_string(),
                "(0, 12.3)".to_string(),
                "(0.1, 23)".to_string(),
                "(0.1, 2.3)".to_string(),
                "(0.12, 3)".to_string()
            ]
        );
        assert_eq!(
            Solution::ambiguous_coordinates("(00011)".to_string()),
            vec!["(0, 0.011)".to_string(), "(0.001, 1)".to_string()]
        );
    }
}
