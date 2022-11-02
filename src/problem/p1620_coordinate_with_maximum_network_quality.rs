// 1620. Coordinate With Maximum Network Quality
// https://leetcode.cn/problems/coordinate-with-maximum-network-quality/

use std::vec;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let (mut x_max, mut y_max) = (0, 0);
        for t in &towers {
            x_max = x_max.max(t[0]);
            y_max = y_max.max(t[1]);
        }
        let (mut cx, mut cy, mut max_quality) = (0, 0, 0);
        for i in 0..=x_max {
            for j in 0..=y_max {
                let mut q = 0;
                // 遍历 towers
                for t in &towers {
                    let distance = { (t[0] - i) * (t[0] - i) + (t[1] - j) * (t[1] - j) };
                    if distance <= radius * radius {
                        // 满足信号塔辐射范围
                        // 计算质量
                        q += ((t[2] as f64) / (1.0 + f64::sqrt(distance as f64))) as i32;
                    }
                }
                if q > max_quality {
                    max_quality = q;
                    cx = i;
                    cy = j;
                }
            }
        }
        vec![cx, cy]
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1620() {
        assert_eq!(
            Solution::best_coordinate(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], 2),
            vec![2, 1]
        );
        assert_eq!(
            Solution::best_coordinate(vec![vec![23, 11, 21]], 9),
            vec![23, 11]
        );
        assert_eq!(
            Solution::best_coordinate(vec![vec![1, 2, 13], vec![2, 1, 7], vec![0, 1, 9]], 2),
            vec![1, 2]
        );
        assert_eq!(
            Solution::best_coordinate(
                vec![
                    vec![31, 13, 33],
                    vec![24, 45, 38],
                    vec![28, 32, 23],
                    vec![7, 23, 22],
                    vec![41, 50, 33],
                    vec![47, 21, 3],
                    vec![3, 33, 39],
                    vec![11, 38, 5],
                    vec![26, 20, 28],
                    vec![48, 39, 16],
                    vec![34, 29, 25]
                ],
                21
            ),
            vec![24, 45]
        );
        assert_eq!(
            Solution::best_coordinate(
                vec![
                    vec![30, 34, 31],
                    vec![10, 44, 24],
                    vec![14, 28, 23],
                    vec![50, 38, 1],
                    vec![40, 13, 6],
                    vec![16, 27, 9],
                    vec![2, 22, 23],
                    vec![1, 6, 41],
                    vec![34, 22, 40],
                    vec![40, 10, 11]
                ],
                20
            ),
            vec![1, 6]
        );
    }
}
