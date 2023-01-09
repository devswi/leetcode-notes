import Foundation

/// https://leetcode-cn.com/problems/single-number/
public func singleNumber(_ nums: [Int]) -> Int {
    var result = nums[0]
    for i in stride(from: 1, to: nums.count, by: 1) {
        result ^= nums[i]
    }
    return result
}
