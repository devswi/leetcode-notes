// 801. Minimum Swaps To Make Sequences Increasing
// https://leetcode.cn/problems/minimum-swaps-to-make-sequences-increasing/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0801() {
        assert_eq!(Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
        assert_eq!(
            Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]),
            1
        );
    }
}
