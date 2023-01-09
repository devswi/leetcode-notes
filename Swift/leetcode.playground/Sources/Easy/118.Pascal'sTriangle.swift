import Foundation

/// https://leetcode-cn.com/problems/pascals-triangle/
public func generate(_ numRows: Int) -> [[Int]] {
    if numRows == 0 { return [] }
    var result: [[Int]] = [[1]]
    // 动态规划 dp[m][n] = dp[m-1][n-1] + dp[m-1][n] (越界返回0)
    for m in stride(from: 1, to: numRows, by: 1) {
        var dp: [Int] = [1]
        for n in stride(from: 1, through: m, by: 1) {
            dp.append(result[m - 1][n - 1] + ((n > m - 1) ? 0 : result[m - 1][n]))
        }
        result.append(dp)
    }
    return result
}
