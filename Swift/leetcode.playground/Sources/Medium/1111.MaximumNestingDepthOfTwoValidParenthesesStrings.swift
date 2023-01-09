import Foundation

public func maxDepthAfterSplit(_ seq: String) -> [Int] {
    var result: [Int] = []
    var depth = 0 // 模拟栈，最大深度即栈的最大长度
    for c in seq {
        if c == "(" {
            depth += 1 // 模拟入栈
            result.append(depth % 2)
        } else {
            result.append(depth % 2)
            depth -= 1 // 模拟出栈
        }
    }
    return result
}
