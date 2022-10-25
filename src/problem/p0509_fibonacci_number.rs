// 0509. Fibonacci Number
// https://leetcode.cn/problems/fibonacci-number/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 || n == 2 {
            return 1;
        }
        Solution::fib(n - 1) + Solution::fib(n - 2)
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0509() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}
