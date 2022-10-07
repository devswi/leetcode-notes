// 0777. Swap Adjacent in LR String
// https://leetcode.cn/problems/swap-adjacent-in-lr-string/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let len = start.len();
        let mut start_idx: usize = 0;
        let mut end_idx: usize = 0;
        // XL => LX L 向左移动
        // RX => XR R 向右移动
        // 去除 X 后相对下标
        // 如果都为 L start_idx 一定大于 end_idx
        // 如果都为 R start_idx 一定小于 end_idx
        while start_idx < len || end_idx < len {
            while start_idx < len && &start[start_idx..start_idx + 1] == "X" {
                start_idx += 1;
            }
            while end_idx < len && &end[end_idx..end_idx + 1] == "X" {
                end_idx += 1;
            }
            if start_idx == len || end_idx == len {
                return start_idx == end_idx;
            }
            if start[start_idx..start_idx + 1] != end[end_idx..end_idx + 1] {
                return false;
            }
            if &start[start_idx..start_idx + 1] == "L" && start_idx < end_idx
                || &start[start_idx..start_idx + 1] == "R" && start_idx > end_idx
            {
                return false;
            }
            start_idx += 1;
            end_idx += 1;
        }
        start_idx == end_idx
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0777() {
        assert!(Solution::can_transform(
            String::from("RXXLRXRXL"),
            String::from("XRLXXRRLX")
        ));
    }
}
