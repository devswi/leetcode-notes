// 0455. Assign Cookies
// https://leetcode.cn/problems/assign-cookies/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut children = g;
        children.sort();
        let mut cookies = s;
        cookies.sort();
        let mut child: usize = 0;
        let mut ptr: usize = 0;
        while child < children.len() && ptr < cookies.len() {
            if children[child] <= cookies[ptr] {
                child += 1;
            }
            ptr += 1;
        }
        child as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0455() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 7, 10], vec![1, 3, 5, 9]),
            3
        );
        assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![]), 0);
        assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![3]), 1);
    }
}
