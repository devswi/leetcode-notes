import Foundation

/// https://leetcode-cn.com/problems/search-insert-position/
public func searchInsert(_ nums: [Int], _ target: Int) -> Int {
    var index = 0
    while index < nums.count {
        let num = nums[index]
        if num >= target {
            return index
        }
        index += 1
    }
    return index
}
