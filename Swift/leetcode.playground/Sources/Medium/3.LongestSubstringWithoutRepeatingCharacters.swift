import Foundation

/// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
public func lengthOfLongestSubstring(_ s: String) -> Int {
    guard !s.isEmpty else { return 0 }
    var candidates = Set<String>()
    var maxLen = 0, currentLen = 0, left = 0
    let array = s.map { String($0) }
    for i in (0 ..< array.count) {
        currentLen += 1
        while candidates.contains(array[i]) {
            candidates.remove(array[left])
            left += 1
            currentLen -= 1
        }
        maxLen = max(maxLen, currentLen)
        candidates.insert(array[i])
    }
    return maxLen
}
