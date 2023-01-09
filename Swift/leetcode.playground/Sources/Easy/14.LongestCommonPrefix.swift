import Foundation

/// https://leetcode-cn.com/problems/longest-common-prefix/
public func longestCommonPrefix(_ strs: [String]) -> String {
    guard !strs.isEmpty else { return "" }
    let str = strs[0]
    let indices = str.indices
    let rest = Array(strs.dropFirst())
    for (offset, indice) in indices.enumerated() {
        let current = str[indice]
        for j in (0..<rest.count) {
            if offset == rest[j].count || (rest[j].indices.contains(indice) && current != rest[j][indice]) {
                return String(str[str.startIndex..<indice])
            }
        }
    }
    return str
}
