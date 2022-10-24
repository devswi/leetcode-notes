// 0733. Flood Fill
// https://leetcode.cn/problems/flood-fill/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn flood_fill(_images: Vec<Vec<i32>>, _sr: i32, _sc: i32, _color: i32) -> Vec<Vec<i32>> {
        unimplemented!();
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0733() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
        );
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        );
    }
}
