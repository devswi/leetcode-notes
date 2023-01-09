import Foundation

/// https://leetcode-cn.com/problems/string-to-integer-atoi/
public func myAtoi(_ str: String) -> Int {
    let maxInt = Int(1<<31 - 1)
    let minInt = -Int(1<<31)
    
    var isNegative: Bool?
    var numbers: [Int] = []
    var result = 0
    var hasNumber = false
    for c in str {
        if numbers.isEmpty && (c == "+" || c == "-") {
            if isNegative == nil && !hasNumber {
                isNegative = c == "-"
            } else {
                return result
            }
        } else {
            let number = Int(c.unicodeScalars.first!.value) - Int("0".unicodeScalars.first!.value)
            if (number > 9 || number < 0) { // 非数字
                if hasNumber || c != " " || isNegative != nil { break }
            } else {
                hasNumber = true
                if number == 0 && numbers.isEmpty { // 最高位为0，忽略
                    continue
                } else {
                    numbers.append(number)
                }
            }
        }
    }
    
    isNegative = isNegative ?? false
    
    if numbers.count > 10 {
        return isNegative! ? minInt : maxInt
    }
    
    let count = numbers.count
    for i in stride(from: 0, to: numbers.count, by: 1) {
        let p = pow(Double(10), Double(count - i - 1))
        let increment = Int(p) * numbers[i]
        if !isNegative! && (maxInt - result) < increment {
            print((maxInt / 10 - result))
            print(increment)
            return maxInt
        } else if isNegative! && (Int(abs(result + minInt)) < increment) {
            return minInt
        } else {
            result += increment
        }
    }
    
    return result * (isNegative! ? -1 : 1)
}
