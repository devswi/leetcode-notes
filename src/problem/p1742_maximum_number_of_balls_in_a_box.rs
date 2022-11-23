// 1742. Maximum Number of Balls in a Box
// https://leetcode.cn/problems/maximum-number-of-balls-in-a-box/

use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut res = 0;
        let mut map = HashMap::new();
        for i in low_limit..=high_limit {
            let mut x = i;
            let mut num = 0;
            while x > 0 {
                num += x % 10;
                x /= 10;
            }
            map.entry(num).and_modify(|x| *x += 1).or_insert(1);
            res = res.max(map[&num]);
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1742() {
        assert_eq!(Solution::count_balls(1, 10), 2);
        assert_eq!(Solution::count_balls(5, 15), 2);
        assert_eq!(Solution::count_balls(19, 28), 2);
    }
}
