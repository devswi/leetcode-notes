// 1710. Maximum Units on a Truck
// https://leetcode.cn/problems/maximum-units-on-a-truck/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut truck_size = truck_size;
        let mut box_types = box_types;
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut res = 0;
        for box_type in box_types {
            let count = box_type[0].min(truck_size);
            let size = box_type[1];
            res += size * count;
            truck_size -= count;
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1710() {
        assert_eq!(
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
            8
        );
        assert_eq!(
            Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10),
            91
        );
    }
}
