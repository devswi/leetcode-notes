// 0754. Reach a Number
// https://leetcode.cn/problems/reach-a-number/description/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        use std::collections::VecDeque;
        let target = target.abs();
        let mut res = 1;
        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut len = queue.len();
        while !queue.is_empty() {
            if let Some(num) = queue.pop_front() {
                len -= 1;
                if num + res == target {
                    return res;
                }
                queue.push_back(num + res);
            }
            if len == 0 {
                len = queue.len();
                res += 1;
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
    fn test_0754() {
        assert_eq!(Solution::reach_number(2), 3);
        assert_eq!(Solution::reach_number(3), 2);
    }
}
