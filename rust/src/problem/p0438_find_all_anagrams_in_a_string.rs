// 0438. Find All Anagrams in a String
// https://leetcode.cn/problems/find-all-anagrams-in-a-string/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        // 窗口滑动
        let mut ans = vec![];
        let (s_len, p_len) = (s.len(), p.len());

        // 初始化，统计 s 中前 p.len 的字符数量
        let mut s_calc = vec![0; 26];
        let s = s.into_bytes();
        for c in 0..(p_len - 1).min(s_len) {
            s_calc[(s[c] - b'a') as usize] += 1;
        }

        // 统计 s 和 p 中字符的数量
        let mut p_calc = vec![0; 26];
        let p = p.into_bytes();
        for c in p {
            p_calc[(c - b'a') as usize] += 1;
        }

        for i in (p_len - 1)..s_len {
            s_calc[(s[i] - b'a') as usize] += 1;
            if p_calc == s_calc {
                ans.push((i + 1 - p_len) as i32);
            }
            s_calc[(s[i + 1 - p_len] - b'a') as usize] -= 1;
        }

        ans
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
