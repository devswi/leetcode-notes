// 0862. Shortest Subarray with Sum at Least K
// https://leetcode.cn/problems/shortest-subarray-with-sum-at-least-k/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn shortest_subarray(_nums: Vec<i32>, _k: i32) -> i32 {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0862() {
        assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
        assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1);
        assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3);
    }
}
