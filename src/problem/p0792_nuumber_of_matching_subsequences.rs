// 801. Minimum Swaps To Make Sequences Increasing
// https://leetcode.cn/problems/minimum-swaps-to-make-sequences-increasing/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut arr = vec![vec![]; 26];
        for (idx, b) in s.bytes().into_iter().enumerate() {
            arr[(b - b'a') as usize].push(idx);
        }
        let mut res = words.len();
        for w in words {
            if w.len() > s.len() {
                res -= 1;
                continue;
            }
            let mut p = -1;
            for b in w.bytes() {
                let idx = (b - b'a') as usize;
                if arr[idx].len() == 0 || (arr[idx][arr[idx]] as i32) <= p {}
            }
        }
        res as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0792() {
        assert_eq!(
            Solution::num_matching_subseq(
                "abced".to_string(),
                vec![
                    "a".to_string(),
                    "bb".to_string(),
                    "acd".to_string(),
                    "ace".to_string()
                ]
            ),
            3
        );
        assert_eq!(
            Solution::num_matching_subseq(
                "dsahjpjauf".to_string(),
                vec![
                    "ahjpjau".to_string(),
                    "ja".to_string(),
                    "ahbwzgqnuk".to_string(),
                    "tnmlanowax".to_string()
                ]
            ),
            2
        );
    }
}
