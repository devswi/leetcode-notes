// 0724. Find Pivot Index
// https://leetcode.cn/problems/find-pivot-index/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        for (idx, _) in nums.iter().enumerate() {
            let left: i32 = nums[..idx].iter().sum();
            let right: i32 = nums[idx + 1..].iter().sum();
            if left == right {
                return idx as i32;
            }
        }
        -1
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0724() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }
}
