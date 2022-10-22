// 0278. First Bad Version
// https://leetcode.cn/problems/first-bad-version/

#[allow(dead_code)]
pub struct Solution {
    bad_version: i32,
}

#[allow(dead_code)]
impl Solution {
    pub fn new(bad_version: i32) -> Self {
        Solution { bad_version }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid; // 答案在 [left, mid] 中
            } else {
                left = mid + 1; // 答案在 [mid + 1, left] 中
            }
        }
        left
    }

    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.bad_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0278() {
        let example = Solution::new(4);
        assert_eq!(example.first_bad_version(5), 4);

        let example = Solution::new(1);
        assert_eq!(example.first_bad_version(1), 1);

        let example = Solution::new(1);
        assert_eq!(example.first_bad_version(3), 1);
    }
}
