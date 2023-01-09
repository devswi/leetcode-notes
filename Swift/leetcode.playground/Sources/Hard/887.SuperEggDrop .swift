import Foundation

/// https://leetcode-cn.com/problems/super-egg-drop/
public func superEggDrop(_ K: Int, _ N: Int) -> Int {
//    struct Record: Hashable {
//        let K: Int
//        let N: Int
//    }
//
//    var dict: [Record: Int] = [:]
//    func dp(K: Int, N: Int) -> Int {
//        if K == 1 { return N }
//        if K == 0 { return 0 }
//
//        let record = Record(K: K, N: N)
//        if let r = dict[record] { return r }
//
//        var res = Int.max
//        for i in stride(from: 1, to: N + 1, by: 1) {
//            res = min(res, max(dp(K: K, N: N - i), dp(K: K - 1, N: i - 1))) + 1
//        }
//
//        dict[record] = res
//        return res
//    }
//
//    return dp(K: K, N: N)
    var dp: [[Int]] = [[Int]](repeating: [Int](repeating: 0, count: N + 1), count: K + 1)

    var m = 0
    while dp[K][m] < N {
        m += 1
        for k in stride(from: 1, through: K, by: 1) {
            dp[k][m] = dp[k][m - 1] + dp[k - 1][m - 1] + 1
        }
    }
    return m
}
