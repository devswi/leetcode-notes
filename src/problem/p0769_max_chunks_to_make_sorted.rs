// 0769. Max Chunks To Make Sorted
// https://leetcode.cn/problems/max-chunks-to-make-sorted/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_chunks_to_stored(arr: Vec<i32>) -> i32 {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0769() {
        assert_eq!(Solution::max_chunks_to_stored(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(Solution::max_chunks_to_stored(vec![1, 0, 2, 3, 4]), 4);
    }
}
