// 1768. Merge Strings Alternately
// https://leetcode.cn/problems/merge-strings-alternately/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1768() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr"
        );
        assert_eq!(
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs"
        );
        assert_eq!(
            Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd"
        );
    }
}
