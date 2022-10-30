// 0784. Letter Case Permutation
// https://leetcode.cn/problems/letter-case-permutation/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = vec![];
        ans.push("".to_string());
        for c in s.chars() {
            let n = ans.len();
            let mut i = 0;
            if c.is_ascii_uppercase() {
                while i < n {
                    ans.push(ans[i].to_owned() + &c.to_ascii_lowercase().to_string());
                    ans[i] = ans[i].to_owned() + &c.to_string();
                    i += 1;
                }
            } else if c.is_ascii_lowercase() {
                while i < n {
                    ans.push(ans[i].to_owned() + &c.to_ascii_uppercase().to_string());
                    ans[i] = ans[i].to_owned() + &c.to_string();
                    i += 1;
                }
            } else {
                // 数字
                while i < n {
                    ans[i] = ans[i].to_owned() + &c.to_string();
                    i += 1;
                }
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
    fn test_0784() {
        assert_eq!(
            Solution::letter_case_permutation("a1b2".to_string()),
            vec![
                "a1b2".to_string(),
                "A1b2".to_string(),
                "a1B2".to_string(),
                "A1B2".to_string()
            ]
        );
        assert_eq!(
            Solution::letter_case_permutation("3z4".to_string()),
            vec!["3z4".to_string(), "3Z4".to_string(),]
        );
    }
}
