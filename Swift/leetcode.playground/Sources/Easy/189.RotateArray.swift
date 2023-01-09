import Foundation

/// https://leetcode-cn.com/problems/rotate-array/
public func rotate(_ nums: inout [Int], _ k: Int) {
    // 暴力法 timeout
    if k == 0 { return }
    for i in stride(from: 0, to: nums.count - 1, by: 1) {
        // 交换下标 i 与 nums.count - 1 的两个数
        nums[i] = nums[i] ^ nums[nums.count - 1]
        nums[nums.count - 1] = nums[i] ^ nums[nums.count - 1]
        nums[i] = nums[i] ^ nums[nums.count - 1]
    }
    rotate(&nums, k - 1)
}

// 三次旋转
public func rotate3Times(_ nums: inout [Int], _ k: Int) {
    var k = k
    k %= nums.count
    func reverse(_ start: Int, _ end: Int) {
        var start = start
        var end = end
        while start < end {
            nums[start] = nums[start] ^ nums[end]
            nums[end] = nums[start] ^ nums[end]
            nums[start] = nums[start] ^ nums[end]
            start += 1
            end -= 1
        }
    }
    // 第一次反转
    reverse(0, nums.count - 1 - k)
    // 第二次反转
    reverse(nums.count - k, nums.count - 1)
    // 第三次反转
    reverse(0, nums.count - 1)
}
