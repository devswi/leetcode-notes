// 1732. Find the Highest Altitude
// https://leetcode.cn/problems/find-the-highest-altitude/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut pre_altitude = 0;
        let mut res = 0;
        for g in gain {
            pre_altitude += g;
            res = res.max(pre_altitude);
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1732() {
        assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
        assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
