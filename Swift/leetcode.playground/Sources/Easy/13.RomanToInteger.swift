import Foundation

/// https://leetcode-cn.com/problems/roman-to-integer/
public func romanToInt(_ s: String) -> Int {
    let dict = [
        "I": 1,
        "IV": 4,
        "V": 5,
        "IX": 9,
        "X": 10,
        "XL": 40,
        "L": 50,
        "XC": 90,
        "C": 100,
        "CD": 400,
        "D": 500,
        "CM": 900,
        "M": 1000
    ]
    let holdOn = ["I", "X", "C"]
    var i = 0
    var result = 0
    while i < s.count {
        let currentIndex = s.index(s.startIndex, offsetBy: i)
        let nextIndex = s.index(after: currentIndex)
        let current = String(s[currentIndex])
        if holdOn.contains(current) && s.indices.contains(nextIndex), let value = dict[String(s[currentIndex...nextIndex])] {
            result += value
            i += 2
        } else {
            result += dict[String(s[currentIndex])] ?? 0
            i += 1
        }
    }
    return result
}
