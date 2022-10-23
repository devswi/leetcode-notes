// 1768. Merge Strings Alternately
// https://leetcode.cn/problems/merge-strings-alternately/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1_len = word1.len();
        let word2_len = word2.len();
        let word1 = word1.chars().collect::<Vec<char>>();
        let word2 = word2.chars().collect::<Vec<char>>();
        let mut ans = vec!["".to_string(); word1_len + word2_len];
        // 双指针
        let mut i = 0;
        let mut j = 0;

        while i < word1_len || j < word2_len {
            if let Some(c) = word1.get(i) {
                ans.push(c.to_string());
                i += 1;
            }
            if let Some(c) = word2.get(j) {
                ans.push(c.to_string());
                j += 1;
            }
        }

        ans.into_iter().collect()
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
