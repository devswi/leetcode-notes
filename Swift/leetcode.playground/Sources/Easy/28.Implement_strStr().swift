import Foundation

/// https://leetcode-cn.com/problems/implement-strstr/
public func strStr(_ haystack: String, _ needle: String) -> Int {
    guard !needle.isEmpty else { return 0 }
    if haystack == needle { return 0 }
    var index = 0
    while (index + needle.count) <= haystack.count {
        let start = haystack.index(haystack.startIndex, offsetBy: index)
        let end = haystack.index(haystack.startIndex, offsetBy: index + needle.count)
        let sub = String(haystack[start..<end])
        if sub == needle {
            return index
        } else {
           index += 1
        }
    }
    return -1
}
