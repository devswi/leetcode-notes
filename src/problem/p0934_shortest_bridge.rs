// 0934. Shortest Bridge
// https://leetcode.cn/problems/shortest-bridge/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn shortest_bridge(_grid: Vec<Vec<i32>>) -> i32 {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0934() {
        assert_eq!(Solution::shortest_bridge(vec![vec![0, 1], vec![1, 0]]), 1);
        assert_eq!(
            Solution::shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::shortest_bridge(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            1
        );
    }
}
