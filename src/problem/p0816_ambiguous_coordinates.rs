// 0816. Ambiguous Coordinates
// https://leetcode.cn/problems/ambiguous-coordinates/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn ambiguous_coordinates(_s: String) -> Vec<String> {
        unimplemented!()
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
                "(1, 2.3)".to_string(),
                "(1, 23)".to_string(),
                "(1.2, 3)".to_string(),
                "(12, 3)".to_string()
            ]
        );
        assert_eq!(
            Solution::ambiguous_coordinates("(0123)".to_string()),
            vec![
                "(0, 1.23)".to_string(),
                "(0, 12.3)".to_string(),
                "(0, 123)".to_string(),
                "(0.1, 2.3)".to_string(),
                "(0.1, 23)".to_string(),
                "(0.12, 3)".to_string()
            ]
        );
        assert_eq!(
            Solution::ambiguous_coordinates("(00011)".to_string()),
            vec!["(0, 0.011)".to_string(), "(0.001, 1)".to_string()]
        );
    }
}
