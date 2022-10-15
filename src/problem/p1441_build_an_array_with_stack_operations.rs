// 1441. Build an Array With Stack Operations
// https://leetcode.cn/problems/build-an-array-with-stack-operations/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut cur: usize = 0;
        let mut res: Vec<String> = vec![];
        for i in 1..=n {
            if i == target[cur] {
                res.push("Push".to_string());
                cur += 1;
            } else {
                res.push("Push".to_string());
                res.push("Pop".to_string());
            }
            if cur >= target.len() {
                break;
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
    fn test_1441() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec!["Push".to_string(), "Push".to_string(), "Push".to_string()]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2], 4),
            vec!["Push".to_string(), "Push".to_string()]
        );
    }
}
