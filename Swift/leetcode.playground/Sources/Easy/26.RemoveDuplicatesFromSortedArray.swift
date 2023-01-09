import Foundation

/// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/
public func removeDuplicates(_ nums: inout [Int]) -> Int {
    guard !nums.isEmpty else { return 0 }
    var index = 1
    while index < nums.count {
        let previous = max(0, index - 1)
        if nums[previous] == nums[index] {
            nums.remove(at: index)
        } else {
            index += 1
        }
    }
    return nums.count
}
