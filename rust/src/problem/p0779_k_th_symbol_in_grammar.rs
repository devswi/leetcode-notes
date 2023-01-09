// 0779. K-th Symbol in Grammar
// https://leetcode.cn/problems/k-th-symbol-in-grammar/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut k = k;
        // n 层有 2 ^ (n - 1) 个节点
        let mut count = 1 << (n - 1);
        let mut cur = 0;
        while count > 1 {
            count >>= 1;
            if k > count {
                k -= count;
                cur ^= 1;
            }
        }
        cur
        // 思路二叉树的层序遍历 timeout
        // use std::collections::VecDeque;
        // // 逐层模拟
        // let mut queue: VecDeque<i32> = VecDeque::new();
        // queue.push_back(0);
        // // 当前层
        // let mut row = 1;
        // // 上一层剩余数量
        // let mut cur = queue.len();
        // while cur > 0 && row < n {
        //     let res = queue.pop_front().unwrap();
        //     if res == 0 {
        //         queue.push_back(0);
        //         queue.push_back(1);
        //     } else {
        //         queue.push_back(1);
        //         queue.push_back(0);
        //     }
        //     cur -= 1;
        //     if cur == 0 {
        //         row += 1;
        //         cur = queue.len();
        //     }
        // }
        // queue[k as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0779() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
        assert_eq!(Solution::kth_grammar(3, 2), 1);
        assert_eq!(Solution::kth_grammar(4, 6), 0);
        assert_eq!(Solution::kth_grammar(30, 434991989), 0);
    }
}
