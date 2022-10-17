// 0904. Fruit Into Baskets
// https://leetcode.cn/problems/fruit-into-baskets/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut res = 0;
        let len = fruits.len();
        // 统计水果数量
        let mut count = vec![0; len];
        let mut total = 0;
        let mut j = 0;
        for (i, f) in fruits.iter().enumerate() {
            count[*f as usize] += 1;
            if count[*f as usize] == 1 {
                total += 1;
            }
            while total > 2 {
                count[fruits[j] as usize] -= 1;
                if count[fruits[j] as usize] == 0 {
                    total -= 1;
                }
                j += 1;
            }
            res = std::cmp::max(res, i - j + 1);
        }
        res as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0904() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
        assert_eq!(
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
            5
        );
    }
}
