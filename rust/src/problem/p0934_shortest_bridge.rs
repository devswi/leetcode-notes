// 0934. Shortest Bridge
// https://leetcode.cn/problems/shortest-bridge/

#[allow(dead_code)]
pub struct Solution {}

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let len = grid.len();
        let mut grid = grid;
        let mut queue = VecDeque::new();
        'exit: for i in 0..len {
            for j in 0..len {
                if grid[i][j] == 1 {
                    Solution::dfs(&mut grid, &mut queue, i, j);
                    break 'exit;
                }
            }
        }

        let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        while let Some((i, j, d)) = queue.pop_front() {
            match grid[i][j] {
                1 => {
                    return d;
                }
                0 | 2 => {
                    grid[i][j] = 3;

                    for &(dx, dy) in &dirs {
                        let x = (i as i32 + dx) as usize;
                        let y = (j as i32 + dy) as usize;

                        if x < len && y < len {
                            queue.push_back((x, y, d + 1))
                        }
                    }
                }
                _ => {}
            }
        }

        0
    }

    fn dfs(
        grid: &mut Vec<Vec<i32>>,
        queue: &mut VecDeque<(usize, usize, i32)>,
        x: usize,
        y: usize,
    ) {
        let len = grid.len();
        if x >= len || y >= len || grid[x][y] == 2 {
            return;
        }
        if grid[x][y] == 0 {
            queue.push_back((x, y, 0));
            return;
        }
        // 标记已访问的土地
        grid[x][y] = 2;
        if x > 0 {
            Solution::dfs(grid, queue, x - 1, y);
        }
        Solution::dfs(grid, queue, x + 1, y);
        if y > 0 {
            Solution::dfs(grid, queue, x, y - 1);
        }
        Solution::dfs(grid, queue, x, y + 1);
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
