import Foundation

/// https://leetcode-cn.com/problems/spiral-matrix-ii/
public func generateMatrix(_ n: Int) -> [[Int]] {
    // border
    var left = 0, right = n - 1, top = 0, bottom = n - 1
    // generate empty n*n matrix
    var matrix = Array(repeating: Array(repeating: 0, count: n), count: n)
    var initial = 1, target = n * n
    while initial <= target {
        // left -> right
        for i in stride(from: left, through: right, by: 1) {
            matrix[top][i] = initial
            initial += 1
        }
        top += 1
        // top -> bottom
        for i in stride(from: top, through: bottom, by: 1) {
            matrix[i][right] = initial
            initial += 1
        }
        right -= 1
        // right -> left
        for i in stride(from: right, through: left, by: -1) {
            matrix[bottom][i] = initial
            initial += 1
        }
        bottom -= 1
        // bottom -> top
        for i in stride(from: bottom, through: top, by: -1) {
            matrix[i][left] = initial
            initial += 1
        }
        left += 1
    }
    return matrix
}
