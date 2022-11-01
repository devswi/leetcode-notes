// 1662. Check If Two String Arrays are Equivalent
// https://leetcode.cn/problems/check-if-two-string-arrays-are-equivalent/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1662() {
        assert!(Solution::array_strings_are_equal(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        ));
        assert!(!Solution::array_strings_are_equal(
            vec!["a".to_string(), "cb".to_string()],
            vec!["ab".to_string(), "c".to_string()]
        ));
    }
}
