import Foundation

/// https://leetcode-cn.com/problems/valid-parentheses/
public func isValid(_ s: String) -> Bool {
    var result: [String] = []
    let indices = s.indices
    for indice in indices {
        let current = String(s[indice])
        switch current {
            case "(": result.append(")")
            case "[": result.append("]")
            case "{": result.append("}")
            default:
                if result.isEmpty { return false }
                if let last = result.popLast(), last != current {
                    return false
                }
        }
    }
    return result.isEmpty
}
