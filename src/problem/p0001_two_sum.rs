// 0001. Two Sum
// https://leetcode.cn/problems/two-sum/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut dict = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            if let Some(res) = dict.get(num) {
                // 匹配到结果
                return vec![*res, idx as i32];
            }
            dict.entry(target - *num).or_insert(idx as i32);
        }
        vec![]
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0001() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
