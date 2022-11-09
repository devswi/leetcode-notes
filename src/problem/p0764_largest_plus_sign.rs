// 0764. Largest Plus Sign
// https://leetcode.cn/problems/largest-plus-sign/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn order_of_largest_plus_sign(_n: i32, _mines: Vec<Vec<i32>>) -> i32 {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0764() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
        assert_eq!(Solution::order_of_largest_plus_sign(1, vec![vec![0, 0]]), 0);
    }
}
