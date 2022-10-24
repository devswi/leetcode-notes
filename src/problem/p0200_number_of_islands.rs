// 0200. Number of Islands
// https://leetcode.cn/problems/number-of-islands/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    Solution::dfs(&mut grid, i, j);
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize) {
        if grid[x][y] != '1' {
            return;
        }
        grid[x][y] = '2';
        if x > 0 {
            Solution::dfs(grid, x - 1, y);
        }
        if x < grid.len() - 1 {
            Solution::dfs(grid, x + 1, y);
        }
        if y > 0 {
            Solution::dfs(grid, x, y - 1);
        }
        if y < grid[0].len() - 1 {
            Solution::dfs(grid, x, y + 1);
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0200() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
