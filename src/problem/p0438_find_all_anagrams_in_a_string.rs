// 0438. Find All Anagrams in a String
// https://leetcode.cn/problems/find-all-anagrams-in-a-string/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_anagrams(_s: String, _p: String) -> Vec<i32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0438() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );
    }
}
