// 1790.  Check if One String Swap Can Make Strings Equal
// https://leetcode.cn/problems/check-if-one-string-swap-can-make-strings-equal/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut diff = vec![];
        for (idx, (ch1, ch2)) in s1.chars().zip(s2.chars()).enumerate() {
            if ch1 != ch2 {
                if diff.len() >= 2 {
                    return false;
                }
                diff.push(idx);
            }
        }
        println!("{:?}", diff);
        if diff.is_empty() {
            return true;
        }
        if diff.len() != 2 {
            return false;
        }
        s1.chars().nth(diff[0]).unwrap() == s2.chars().nth(diff[1]).unwrap()
            && s1.chars().nth(diff[1]).unwrap() == s2.chars().nth(diff[0]).unwrap()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1790() {
        assert!(Solution::are_almost_equal(
            String::from("bank"),
            String::from("kanb")
        ));
        assert!(!Solution::are_almost_equal(
            String::from("attack"),
            String::from("defend")
        ));
        assert!(Solution::are_almost_equal(
            String::from("kelb"),
            String::from("kelb")
        ));
    }
}
