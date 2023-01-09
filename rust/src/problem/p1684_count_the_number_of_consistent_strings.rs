// 1684. Count the Number of Consistent Strings
// https://leetcode.cn/problems/count-the-number-of-consistent-strings/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut ans = 0;
        for w in words {
            let mut matched = true;
            for c in w.chars() {
                if !allowed.contains(c) {
                    matched = false;
                    break;
                }
            }
            if matched {
                ans += 1;
            }
        }
        ans
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1684() {
        assert_eq!(
            Solution::count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            Solution::count_consistent_strings(
                "abc".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string(),
                ]
            ),
            7
        );
    }
}
