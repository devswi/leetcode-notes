import Foundation

public func climbStairs(_ n: Int) -> Int {
    if n == 1 { return 1 }
    var dp = Array(repeating: 0, count: n + 1)
    dp[1] = 1
    dp[2] = 2
    for i in stride(from: 3, through: n, by: 1) {
        dp[i] = dp[i - 1] + dp[i - 2]
    }
    return dp[n]
}
