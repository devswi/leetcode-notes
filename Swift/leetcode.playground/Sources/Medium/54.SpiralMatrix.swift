import Foundation

/// https://leetcode-cn.com/problems/spiral-matrix/
public func spiralOrder(_ matrix: [[Int]]) -> [Int] {
    let m = matrix.count // 行
    guard let n = matrix.first?.count else { return [] }
    var result: [Int] = Array(repeating: 0, count: m * n)
    var top = 0, left = 0, bottom = m - 1, right = n - 1
    var initial = 0
    let target = m * n - 1
    while initial <= target {
        // left -> right
        for i in stride(from: left, through: right, by: 1) {
            result[initial] = matrix[top][i]
            initial += 1
        }
        top += 1
        if top > bottom { break }
        // top -> bottom
        for i in stride(from: top, through: bottom, by: 1) {
            result[initial] = matrix[i][right]
            initial += 1
        }
        right -= 1
        if right < left { break }
        // right -> left
        for i in stride(from: right, through: left, by: -1) {
            result[initial] = matrix[bottom][i]
            initial += 1
        }
        bottom -= 1
        if bottom < top { break }
        // bottom -> top
        for i in stride(from: bottom, through: top, by: -1) {
            result[initial] = matrix[i][left]
            initial += 1
        }
        left += 1
        if left > right { break }
    }
    return result
}
