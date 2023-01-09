import Foundation

/// https://leetcode-cn.com/problems/pascals-triangle-ii/
public func getRow(_ rowIndex: Int) -> [Int] {
    var result: [Int] = [1]
    for i in stride(from: 0, to: rowIndex, by: 1) {
        result = [1] + result
        for j in stride(from: 1, through: i, by: 1) {
            result[j] = result[j] + result[j + 1]
        }
    }
    return result
}
