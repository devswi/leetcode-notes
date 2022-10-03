// 1784. Check if Binary String Has at Most One Segment of Ones
// https://leetcode.cn/problems/check-if-binary-string-has-at-most-one-segment-of-ones/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let split_res: Vec<&str> = s.split('0').collect();
        let mut count = 0;
        for w in split_res {
            if !w.is_empty() {
                count += 1;
            }
        }
        count <= 1
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1784() {
        assert!(!Solution::check_ones_segment(String::from("1001")));
        assert!(Solution::check_ones_segment(String::from("110")));
        assert!(!Solution::check_ones_segment(String::from("1100111")));
    }
}
