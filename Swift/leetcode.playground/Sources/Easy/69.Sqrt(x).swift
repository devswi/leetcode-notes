import Foundation

/// https://leetcode-cn.com/problems/sqrtx/
public func mySqrt(_ x: Int) -> Int {
    var minBorder = min(1, x)
    var maxBorder = max(1, x)
    while true {
        let result = (maxBorder - minBorder) / 2 + minBorder
        if minBorder * minBorder == x || result == minBorder || result == maxBorder {
            break
        } else if result * result > x {
            maxBorder = result
        } else {
            minBorder = result
        }
    }
    return minBorder
}
