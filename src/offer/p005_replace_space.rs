// 剑指 Offer 05. 替换空格
// https://leetcode.cn/problems/ti-huan-kong-ge-lcof/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut res = vec![];
        for c in s.chars() {
            if c == ' ' {
                res.push('%');
                res.push('2');
                res.push('0');
            } else {
                res.push(c);
            }
        }
        res.into_iter().collect()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_005() {
        assert_eq!(
            Solution::replace_space(String::from("We are happy.")),
            String::from("We%20are%20happy.")
        );
    }
}
