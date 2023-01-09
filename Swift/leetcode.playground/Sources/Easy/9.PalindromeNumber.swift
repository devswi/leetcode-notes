import Foundation

/// https://leetcode-cn.com/problems/palindrome-number/
public func isPalindrome(_ x: Int) -> Bool {
    guard x > 0 || x % 10 == 0 else { return false }
    var original = x, result = 0
    while original != 0 {
        let last = original % 10
        result = result * 10 + last
        original /= 10
    }
    return x == result
}
