import Foundation

/// https://leetcode-cn.com/problems/minimum-increment-to-make-array-unique/
public func minIncrementForUnique(_ A: [Int]) -> Int {
    // 排序: 计算差值
    var A = A.sorted()
    var ans = 0
    for i in stride(from: 1, to: A.count, by: 1) {
        if A[i] <= A[i - 1] {
            let pre = A[i]
            A[i] = A[i - 1] + 1
            ans += A[i] - pre
        }
    }
    return ans
    
    // 暴力法: 超时
    var result = 0
    var dict: [Int: Int] = [:]
    for (index, element) in A.enumerated() {
        var element = element
        while dict[element] != nil {
            element += 1
            result += 1
        }
        dict[element] = index
    }
    return result
}
