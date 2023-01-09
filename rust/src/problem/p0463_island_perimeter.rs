// 0463. Island Perimeter
// https://leetcode.cn/problems/island-perimeter/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut lands = 0;
        let mut border = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    lands += 1;
                    if i < grid.len() - 1 && grid[i + 1][j] == 1 {
                        border += 1;
                    }
                    if j < grid[0].len() - 1 && grid[i][j + 1] == 1 {
                        border += 1;
                    }
                }
            }
        }
        4 * lands - 2 * border
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0463() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
        assert_eq!(Solution::island_perimeter(vec![vec![1],]), 4);
    }
}
