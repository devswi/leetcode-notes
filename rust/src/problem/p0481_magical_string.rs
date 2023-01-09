// 0481. Magical String
// https://leetcode.cn/problems/magical-string/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        use std::char::from_digit;
        let n = n as usize;
        if n < 4 {
            return 1;
        }
        let mut ans = vec!['0'; n];
        ans[1] = '2';
        ans[2] = '2';
        let mut res = 1;
        let mut i = 2;
        let mut j = 3;
        while j < n {
            let mut size = (ans[i] as u8) - b'0';
            let num = 3 - (ans[j - 1] as u8 - b'0');
            while size > 0 && j < n {
                ans[j] = from_digit(num as u32, 10).unwrap();
                if num == 1 {
                    res += 1;
                }
                j += 1;
                size -= 1
            }
            i += 1;
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0481() {
        assert_eq!(Solution::magical_string(6), 3);
        assert_eq!(Solution::magical_string(4), 2);
        assert_eq!(Solution::magical_string(1), 1);
    }
}
