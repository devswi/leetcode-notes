// 1822. Sign of the Product of an Array
// https://leetcode.cn/problems/sign-of-the-product-of-an-array/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut is_negative = false;

        for num in nums {
            if num == 0 {
                return 0;
            }
            if num < 0 {
                is_negative = !is_negative;
            }
        }

        if is_negative {
            -1
        } else {
            1
        }
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1822() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
        assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
        assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
