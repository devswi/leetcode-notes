// 801. Minimum Swaps To Make Sequences Increasing
// https://leetcode.cn/problems/minimum-swaps-to-make-sequences-increasing/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut arr = vec![vec![]; 26];
        s.bytes().into_iter().enumerate().for_each(|(idx, b)| {
            arr[(b - b'a') as usize].push(idx);
        });
        words
            .iter()
            .filter(|&x| Solution::binary_search(x.as_bytes(), &arr))
            .count() as i32
    }

    fn binary_search(x: &[u8], list: &[Vec<usize>]) -> bool {
        let mut pos = 0;
        for ch in x.iter().copied() {
            let ch = (ch - b'a') as usize;
            let loc = list[ch].binary_search(&pos).unwrap_or_else(|e| e);

            if list[ch].len() <= loc {
                return false;
            }

            let idx = list[ch][loc];
            pos = idx + 1;
        }
        true
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
