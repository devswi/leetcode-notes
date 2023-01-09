import Foundation

/// https://leetcode-cn.com/problems/reverse-integer/
public func reverse(_ x: Int) -> Int {
    var answer = 0
    var x = x
    let maxValue = Int(1<<31 - 1) / 10
    let minValue = -Int(1<<31) / 10
    while x != 0 {
        let last = x % 10
        if answer > maxValue || (answer == maxValue  && last > 7) {
            return 0
        }
        if answer < minValue || (answer == minValue && last < -8) {
            return 0
        }
        answer = answer * 10 + last
        x = x / 10
    }
    return answer
}
