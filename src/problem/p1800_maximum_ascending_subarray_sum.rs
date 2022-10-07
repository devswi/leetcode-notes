// 1800. Maximum Ascending Subarray Sum
// https://leetcode.cn/problems/maximum-ascending-subarray-sum/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut pre = 0;
        let mut tmp = 0;
        for n in nums {
            if n > pre {
                tmp += n;
            } else {
                res = std::cmp::max(res, tmp);
                tmp = n;
            }
            pre = n;
        }
        res = std::cmp::max(res, tmp);
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1800() {
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
        assert_eq!(
            Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]),
            33
        );
        assert_eq!(
            Solution::max_ascending_sum(vec![3, 6, 10, 1, 8, 9, 9, 8, 9]),
            19
        );
    }
}
