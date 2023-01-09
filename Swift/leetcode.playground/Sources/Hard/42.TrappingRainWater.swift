import Foundation

/// https://leetcode-cn.com/problems/trapping-rain-water/
public func trap(_ height: [Int]) -> Int {
    var total = 0
    var stack: [Int] = []
    
    for i in stride(from: 0, to: height.count, by: 1) {
        /// 栈不为空 且 当前高度大于栈顶存储下标的高度
        while !stack.isEmpty && height[i] > height[stack.last!] {
            let h = height[stack.last!] // 栈顶下标高度
            stack.removeLast() // 出栈操作
            if stack.isEmpty { break } // 如果栈空，退出
            let distance = i - stack.last! - 1 // 计算两个边界直接的距离
            let minHeight = min(height[stack.last!], height[i]) // 当前栈顶高度与当前遍历的高度，取最小值
            total += distance * (minHeight - h)
        }
        
        stack.append(i) // 入栈
    }
    
    return total
}
