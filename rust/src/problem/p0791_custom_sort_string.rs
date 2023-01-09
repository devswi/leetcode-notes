// 791. Custom Sort String
// https://leetcode.cn/problems/custom-sort-string/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut arr = vec![0; 26];
        for c in s.chars() {
            arr[(c as u8 - b'a') as usize] += 1;
        }
        let mut res = String::new();
        for c in order.chars() {
            let index = (c as u8 - b'a') as usize;
            let count = arr[index];
            res.push_str(&c.to_string().repeat(count));
            arr[index] = 0;
        }
        for (index, rest) in arr.iter().enumerate() {
            if *rest > 0 {
                res.push_str(&((index as u8 + b'a') as char).to_string().repeat(*rest));
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
    fn test_0791() {
        assert_eq!(
            Solution::custom_sort_string("cba".to_string(), "abcd".to_string()),
            "cbad".to_string()
        );
        assert_eq!(
            Solution::custom_sort_string("cbafg".to_string(), "abcd".to_string()),
            "cbad".to_string()
        );
    }
}
