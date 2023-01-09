import Foundation

/// https://leetcode-cn.com/problems/generate-parentheses/
public func generateParenthesis(_ n: Int) -> [String] {
//    var ans = [String]()
//    recursion(ans: &ans, current: "", open: 0, close: 0, max: n)
//    return ans
    if n == 0 { return [""] }
    var ans = [String]()
    for i in stride(from: 0, to: n, by: 1) {
        for left in generateParenthesis(i) {
            for right in generateParenthesis(n - 1 - i) {
                ans.append("(\(left))\(right)")
            }
        }
    }
    return ans
}

func recursion(ans: inout [String], current: String, open: Int, close: Int, max: Int) {
    if current.count == max * 2 {
        ans.append(current)
        return
    }
    
    if open < max {
        recursion(ans: &ans, current: current + "(", open: open + 1, close: close, max: max)
    }
    if close < open {
        recursion(ans: &ans, current: current + ")", open: open, close: close + 1, max: max)
    }
}
