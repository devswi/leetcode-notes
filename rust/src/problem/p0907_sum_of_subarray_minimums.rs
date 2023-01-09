// 0907. Sum of Subarray Minimums
// https://leetcode.cn/problems/sum-of-subarray-minimums/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.push(-1);
        let mut ans = 0;
        let mut st = vec![-1i64];
        for (i, &num) in arr.iter().enumerate() {
            let i = i as i64;
            while st.len() > 1 && arr[*st.last().unwrap() as usize] >= num {
                let j = st.pop().unwrap();
                ans += arr[j as usize] as i64 * (j - *st.last().unwrap()) * (i - j);
            }
            st.push(i as i64);
        }
        (ans % 1_000_000_007) as i32
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0922() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
}
