// 0886. Possible Bipartition
// https://leetcode.cn/problems/possible-bipartition/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let len = (n + 1) as usize;
        // 0: none, 1: red, 2: blue
        let mut color: Vec<usize> = vec![0; len];
        let mut graph: Vec<Vec<usize>> = vec![vec![]; len];
        // 对 dislikes 进行分组
        for dislike in dislikes.iter() {
            graph[dislike[0] as usize].push(dislike[1] as usize);
            graph[dislike[1] as usize].push(dislike[0] as usize);
        }
        for i in 1..=(n as usize) {
            // color 为 0 表示未分组
            if color[i] == 0 && !Solution::dfs(i, 1, &mut color, &graph) {
                return false;
            }
        }
        true
    }

    fn dfs(node: usize, cur_color: usize, color: &mut Vec<usize>, graph: &Vec<Vec<usize>>) -> bool {
        color[node] = cur_color;
        for next in graph[node].iter() {
            // 已上色，有冲突，return false 不满足
            if color[*next] != 0 && color[*next] == color[node] {
                return false;
            }
            // 未分类
            if color[*next] == 0 && !Solution::dfs(*next, 3 ^ cur_color, color, graph) {
                return false;
            }
        }
        true
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_09886() {
        assert!(Solution::possible_bipartition(
            4,
            vec![vec![1, 2], vec![1, 3], vec![2, 4]]
        ));
        assert!(!Solution::possible_bipartition(
            3,
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        ));
        assert!(!Solution::possible_bipartition(
            5,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]
        ));
    }
}
