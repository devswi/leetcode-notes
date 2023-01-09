import Foundation

/// https://leetcode-cn.com/problems/container-with-most-water/
public func maxArea(_ height: [Int]) -> Int {
    // 双指针法，两边同时遍历
    var left = 0
    var right = height.count - 1
    var maxResult = 0
    
    while left < right {
        let leftHeight = height[left]
        let rightHeight = height[right]
        
        let distance = right - left // 容器两边距离
        let minHeight = min(leftHeight, rightHeight) // 水量取决于最短的板

        maxResult = max(maxResult, minHeight * distance)
        
        if leftHeight > rightHeight {
            right -= 1
        } else {
            left += 1
        }
    }
    
    return maxResult
}
