// 1668. Maximum Repeating Substring
// https://leetcode.cn/problems/maximum-repeating-substring/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut res = sequence.len() / word.len();
        while res > 0 {
            if sequence.contains(&word.repeat(res)) {
                break;
            }
            res -= 1;
        }
        res as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1668() {
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ab".to_string()),
            2
        );
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ba".to_string()),
            1
        );
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ac".to_string()),
            0
        );
        assert_eq!(Solution::max_repeating("a".to_string(), "a".to_string()), 1);
        assert_eq!(
            Solution::max_repeating(
                "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string(),
                "aaaba".to_string()
            ),
            5
        );
    }
}
