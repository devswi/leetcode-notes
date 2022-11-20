// 799. Champagne Tower
// https://leetcode.cn/problems/champagne-tower/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut layer = vec![poured as f64];
        for i in 1..=query_row {
            let mut next_layer = vec![0f64; (i + 1) as usize];
            for (index, &rest) in layer.iter().enumerate() {
                if rest > 1f64 {
                    // 还有剩余
                    next_layer[index] += (rest - 1f64) * 0.5;
                    next_layer[index + 1] += (rest - 1f64) * 0.5;
                }
            }
            layer = next_layer;
        }
        layer[query_glass as usize].min(1f64)
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0799() {
        assert_eq!(Solution::champagne_tower(1, 1, 1), 0.0f64);
        assert_eq!(Solution::champagne_tower(2, 1, 1), 0.5f64);
        assert_eq!(Solution::champagne_tower(100000009, 33, 17), 1f64);
    }
}
