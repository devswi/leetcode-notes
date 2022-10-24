// 0915. Partition Array into Disjoint Intervals
// https://leetcode.cn/problems/partition-array-into-disjoint-intervals/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn partition_disjoint(_nums: Vec<i32>) -> i32 {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0915() {
        assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
        assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
    }
}
