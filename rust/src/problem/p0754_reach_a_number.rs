// 0754. Reach a Number
// https://leetcode.cn/problems/reach-a-number/description/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut target = target.abs();
        let mut res = 0;
        while target > 0 {
            res += 1;
            target -= res;
        }
        if target % 2 == 0 {
            res
        } else {
            res + 1 + res % 2
        }
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
