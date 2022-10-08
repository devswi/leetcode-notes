// 0870. Advantage Shuffle
// https://leetcode.cn/problems/advantage-shuffle/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len = nums1.len();
        let mut res = vec![0; len];
        // sort nums1 and nums2 index
        let mut nums1 = nums1;
        nums1.sort();
        let mut idx2 = vec![0; len];
        (0..len).for_each(|i| {
            idx2[i] = i;
        });
        idx2.sort_by(|i, j| nums2[*i].cmp(&nums2[*j]));
        // nums2 ptr
        let mut left: usize = 0;
        let mut right: usize = idx2.len() - 1;
        for num in nums1 {
            if num > nums2[idx2[left]] {
                // nums1[i] > nums2[i]
                res[idx2[left]] = num;
                left += 1;
            } else {
                // nums1[i] <= nums2[i]
                res[idx2[right]] = num;
                right -= 1;
            }
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0870() {
        assert_eq!(
            Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
            vec![2, 11, 7, 15]
        );
        assert_eq!(
            Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
            vec![24, 32, 8, 12]
        );
        assert_eq!(
            Solution::advantage_count(vec![2, 0, 4, 1, 2], vec![1, 3, 0, 0, 2]),
            vec![2, 0, 1, 2, 4]
        );
    }
}
