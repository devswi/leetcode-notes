// 剑指 Offer 04. 二维数组中的查找
// https://leetcode.cn/problems/er-wei-shu-zu-zhong-de-cha-zhao-lcof/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;
        // 二维数组逆时针旋转45°
        // 行
        let mut row: i32 = (matrix.len() - 1) as i32;
        // 列
        let mut column = 0;
        // matrix[row][column] > target 目标一定在左边 row--
        // matrix[row][column] < target 目标一定在右边 column++
        while row >= 0 && column < matrix[0].len() {
            match matrix[row as usize][column].cmp(&target) {
                Ordering::Greater => {
                    row -= 1;
                }
                Ordering::Less => {
                    column += 1;
                }
                _ => return true,
            }
        }
        false
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_004() {
        assert!(Solution::find_number_in2_d_array(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ));
        assert!(!Solution::find_number_in2_d_array(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            20
        ));
        assert!(!Solution::find_number_in2_d_array(vec![vec![]], 20));
    }
}
