// 0424. Longest Repeating Character Replacement
// https://leetcode.cn/problems/longest-repeating-character-replacement/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        // 窗口滑动
        let mut res = 0;
        let s = s.into_bytes();
        let len = s.len();
        let mut left = 0;
        let mut right = 0;
        let mut calc = vec![0; 26];
        let mut max_gap = 0;
        while right < len {
            calc[(s[right] - b'A') as usize] += 1;
            max_gap = max_gap.max(calc[(s[right] - b'A') as usize]);
            right += 1;

            if (right - left) as i32 > max_gap + k {
                calc[(s[left] - b'A') as usize] -= 1;
                left += 1;
            }

            res = res.max((right - left) as i32)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0424() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }
}
