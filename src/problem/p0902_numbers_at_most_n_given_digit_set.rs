// 0902. Numbers At Most N Given Digit Set
// https://leetcode.cn/problems/numbers-at-most-n-given-digit-set/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0902() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(
                vec![
                    "1".to_string(),
                    "3".to_string(),
                    "5".to_string(),
                    "7".to_string()
                ],
                100
            ),
            20
        );
        assert_eq!(
            Solution::at_most_n_given_digit_set(
                vec!["1".to_string(), "4".to_string(), "9".to_string(),],
                1000000000
            ),
            29523
        );
        assert_eq!(
            Solution::at_most_n_given_digit_set(vec!["7".to_string()], 8),
            1
        );
    }
}
