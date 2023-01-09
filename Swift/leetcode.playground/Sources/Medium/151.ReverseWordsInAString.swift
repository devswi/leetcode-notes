import Foundation

/// https://leetcode-cn.com/problems/reverse-words-in-a-string/
public func reverseWords(_ s: String) -> String {
    return s.split(whereSeparator: { $0.isWhitespace }).reversed().joined(separator:" ")
}
