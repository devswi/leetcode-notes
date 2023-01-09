// 1700. Number of Students Unable to Eat Lunch
// https://leetcode.cn/problems/number-of-students-unable-to-eat-lunch/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut s1: i32 = students.iter().sum();
        let mut s0 = (students.len() as i32) - s1;
        for i in sandwiches {
            if i == 0 && s0 > 0 {
                s0 -= 1;
            } else if i == 1 && s1 > 0 {
                s1 -= 1
            } else {
                break;
            }
        }
        s0 + s1
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1700() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
