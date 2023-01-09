// 0927. Three equals parts
// https://leetcode.cn/problems/three-equal-parts/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let none = vec![-1, -1];
        let len = arr.len() as i32;
        let sum = {
            let mut sum = 0;
            for ele in &arr {
                sum += *ele;
            }
            sum
        };
        if sum == 0 {
            return vec![0, len - 1];
        }
        if sum % 3 != 0 {
            return none;
        }
        // 每一部分 1 的数量
        let partial = sum / 3;
        let mut first: usize = 0;
        let mut second: usize = 0;
        let mut third: usize = 0;
        let mut cur = 0;
        for (idx, ele) in arr.iter().enumerate() {
            if *ele == 1 {
                if cur == 0 {
                    first = idx;
                } else if cur == partial {
                    second = idx;
                } else if cur == 2 * partial {
                    third = idx;
                }
                cur += 1;
            }
        }
        let len = arr.len() - third;
        if first + len <= second && second + len <= third {
            let mut i: usize = 0;
            while third + i < arr.len() {
                if arr[i + first] != arr[second + i] || arr[first + i] != arr[third + i] {
                    return none;
                }
                i += 1;
            }
            return vec![(first + len - 1) as i32, (second + len) as i32];
        }
        none
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0927() {
        assert_eq!(Solution::three_equal_parts(vec![1, 0, 1, 0, 1]), vec![0, 3]);
        assert_eq!(
            Solution::three_equal_parts(vec![1, 1, 0, 1, 1]),
            vec![-1, -1]
        );
        assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 1]), vec![0, 2]);
    }
}
