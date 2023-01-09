import Foundation

/// https://leetcode-cn.com/problems/rotate-matrix-lcci/
public func rotate(_ matrix: inout [[Int]]) {
    let length = matrix.count
    
    // step 1 对角线翻转
    for i in stride(from: 0, to: length, by: 1) {
        for j in stride(from: i + 1, to: length, by: 1) {
            matrix[i][j] = matrix[i][j] ^ matrix[j][i]
            matrix[j][i] = matrix[i][j] ^ matrix[j][i]
            matrix[i][j] = matrix[i][j] ^ matrix[j][i]
        }
    }
    
    // step 2 沿中心翻转
    let middle = length >> 1
    for i in stride(from: 0, to: length, by: 1) {
        for j in stride(from: 0, to: middle, by: 1) {
            matrix[i][j] = matrix[i][j] ^ matrix[i][length - 1 - j]
            matrix[i][length - 1 - j] = matrix[i][j] ^ matrix[i][length - 1 - j]
            matrix[i][j] = matrix[i][j] ^ matrix[i][length - 1 - j]
        }
    }
}
