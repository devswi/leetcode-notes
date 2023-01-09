import Foundation

/// https://leetcode-cn.com/problems/maximum-subarray/
public func maxSubArray(_ nums: [Int]) -> Int {
    guard !nums.isEmpty else { return -1 }
    var result = nums[0]
    var dp = result
    nums[1...].forEach { num in
        dp = max(dp + num, num)
        result = max(dp, result)
    }
    return result
}
