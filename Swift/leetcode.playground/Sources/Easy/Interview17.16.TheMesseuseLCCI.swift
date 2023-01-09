import Foundation

/// https://leetcode-cn.com/problems/the-masseuse-lcci/
public func massage(_ nums: [Int]) -> Int {
//    if nums.isEmpty { return 0 }
//    if nums.count == 1 { return nums[0] }
//    var dp: [Int] = Array(repeating: 0, count: nums.count)
//    dp[0] = nums[0]
//    dp[1] = max(dp[0], nums[1])
//    for i in stride(from: 2, to: nums.count, by: 1) {
//        dp[i] = max(dp[i - 1], dp[i - 2] + nums[i])
//    }
//    return dp[nums.count - 1]
    var a = 0, b = 0
    for i in stride(from: 0, to: nums.count, by: 1) {
        let c = max(b, a + nums[i])
        a = b
        b = c
    }
    return b
}
