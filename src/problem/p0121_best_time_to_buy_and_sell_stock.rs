// 0121. Best Time to Buy and Sell Stock
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min = i32::MAX;
        // 遍历数组
        for p in prices {
            if p < min {
                // 发现更低的价格，买
                min = p;
            } else if p - min > profit {
                // 当前收益大于过往收益
                profit = p - min;
            }
        }
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0121() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
