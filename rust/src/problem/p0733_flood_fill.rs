// 0733. Flood Fill
// https://leetcode.cn/problems/flood-fill/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut images = image;
        let sr = sr as usize;
        let sc = sc as usize;
        let origin_color = images[sr][sc];
        if images[sr][sc] != color {
            Solution::helper(&mut images, sr, sc, color, origin_color);
        }
        images
    }

    fn helper(images: &mut Vec<Vec<i32>>, x: usize, y: usize, color: i32, origin_color: i32) {
        if images[x][y] == origin_color {
            images[x][y] = color;
            if x > 0 {
                Solution::helper(images, x - 1, y, color, origin_color);
            }
            if x < images.len() - 1 {
                Solution::helper(images, x + 1, y, color, origin_color);
            }
            if y > 0 {
                Solution::helper(images, x, y - 1, color, origin_color);
            }
            if y < images[0].len() - 1 {
                Solution::helper(images, x, y + 1, color, origin_color);
            }
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0733() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
        );
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        );
    }
}
