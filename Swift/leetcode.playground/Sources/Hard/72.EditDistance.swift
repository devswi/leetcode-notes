import Foundation

/// https://leetcode-cn.com/problems/edit-distance/
public func minDistance(_ word1: String, _ word2: String) -> Int {
    // 动态规划
    
    let l1 = word1.count
    let l2 = word2.count
    
    // 初始化 [l1 + 1][l2 + 1] 长度的数组，考虑空字符串的情况
    var dp = Array<[Int]>(repeating: Array<Int>(repeating: 0, count: l2 + 1), count: l1 + 1)
    
    // 如果此时 word2 为空，那么结果就是 word1 的长度 l1
    for i in stride(from: 0, through: l1, by: 1) {
        dp[i][0] = i
    }
    
    // 如果此时 word1 为空，那么结果就是 word2 的长度 l2
    for j in stride(from: 0, through: l2, by: 1) {
        dp[0][j] = j
    }
    
    // dp[i][j] 表示 word1 中前 i 个字符转变为 word2 中前 j 个字符的最小编辑距离
    // if word1[i] == word2[j]
    //     dp[i][j] = dp[i - 1][j - 1]
    // else
    //     dp[i][j] = min(dp[i - 1][j - 1], dp[i - 1][j], dp[i][j - 1]) + 1
    //
    // 其中
    //  dp[i - 1][j - 1] => 替换操作
    //  dp[i - 1][j] => 删除操作
    //  dp[i][j - 1] => 新增操作
    for i in stride(from: 1, through: l1, by: 1) {
        let index1 = word1.index(word1.startIndex, offsetBy: i - 1)
        for j in stride(from: 1, through: l2, by: 1) {
            let index2 = word2.index(word1.startIndex, offsetBy: j - 1)
            if word1[index1] == word2[index2] {
                dp[i][j] = dp[i - 1][j - 1]
            } else {
                dp[i][j] = min(dp[i - 1][j - 1], min(dp[i - 1][j], dp[i][j - 1])) + 1
            }
        }
    }
    
    return dp[l1][l2]
}
