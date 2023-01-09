// 1064. Last Stone Weight
// https://leetcode.cn/problems/last-stone-weight/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        while stones.len() > 1 {
            stones.sort_by(|a, b| b.cmp(a));
            if stones[0] == stones[1] {
                stones.remove(0);
            } else {
                stones[1] = (stones[0] - stones[1]).abs();
            }
            stones.remove(0);
        }
        if stones.is_empty() {
            0
        } else {
            stones[0]
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1064() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }
}
