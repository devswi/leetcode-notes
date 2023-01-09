import Foundation

/// https://leetcode-cn.com/problems/plus-one/
public func plusOne(_ digits: [Int]) -> [Int] {
    var result = digits
    for i in stride(from: digits.count - 1, through: 0, by: -1) {
        let num = digits[i]
        // 1. 当前为个位 +1 < 10 return
        // 2. 当前不为个位 +1 < 10 return
        // Attention: 列出简化小方法
        if num + 1 < 10 {
            // 当前位置不存在进位时，直接 return
            result[i] = num + 1
            return result
        } else {
            result[i] = 0
        }
    }
    // 出现了数组中所有数据都需要进位的情况，数据 item 添加一个 1 这里有新的内存开销
    return [1] + result
}
