// 0775. Global and Local Inversions
// https://leetcode.cn/problems/global-and-local-inversions/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut max_num = 0;
        for i in 2..nums.len() {
            max_num = max_num.max(nums[i - 2]);
            if max_num > nums[i] {
                return false;
            }
        }
        true
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0775() {
        assert!(Solution::is_ideal_permutation(vec![1, 0, 2]));
        assert!(!Solution::is_ideal_permutation(vec![1, 2, 0]));
    }
}
