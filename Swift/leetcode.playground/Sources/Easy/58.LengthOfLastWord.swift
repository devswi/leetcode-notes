import Foundation

/// https://leetcode-cn.com/problems/length-of-last-word/
public func lengthOfLastWord(_ s: String) -> Int {
    var length = 0
    let endIndex = s.endIndex
    var end: Int = -1

    while (abs(end) - 1) < s.count && s[s.index(endIndex, offsetBy: end)] == " " {
        end -= 1
    }
    if abs(end) - 1 == s.count { return 0 }
    var start = end
    while abs(start) - 1 < s.count && s[s.index(endIndex, offsetBy: start)] != " " {
        start -= 1
    }
    return end - start
}
