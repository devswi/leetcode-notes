// 1694. Reformat Phone Number
// https://leetcode.cn/problems/reformat-phone-number/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn reformat_number(number: String) -> String {
        let number: String = number
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect::<String>();
        let mut res: String = String::new();
        let mut len = number.len();
        let mut cur: usize = 0;
        while len > 0 {
            if len == 4 {
                res.push_str(&number[cur..cur + 2]);
                res.push('-');
                res.push_str(&number[cur + 2..]);
                break;
            } else {
                let sub = std::cmp::min(3, len);
                let sub_part: &str = &number[cur..sub + cur];
                res.push_str(sub_part);
                if len > 4 {
                    res.push('-');
                }
                cur += sub;
                len -= sub;
            }
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1694() {
        assert_eq!(
            Solution::reformat_number(String::from("12")),
            String::from("12")
        );
        assert_eq!(
            Solution::reformat_number(String::from("1-23-45 6")),
            String::from("123-456")
        );
        assert_eq!(
            Solution::reformat_number(String::from("123 4-567")),
            String::from("123-45-67")
        );
        assert_eq!(
            Solution::reformat_number(String::from("123 4-5678")),
            String::from("123-456-78")
        );
    }
}
