// 0179. Largest Number
// https://leetcode.cn/problems/largest-number/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_by(|a, b| format!("{b}{a}").cmp(&format!("{a}{b}")));
        if nums[0] == 0 {
            return 0.to_string();
        }
        nums.into_iter().map(|i| i.to_string()).collect::<String>()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0179() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_string()
        );
    }
}
