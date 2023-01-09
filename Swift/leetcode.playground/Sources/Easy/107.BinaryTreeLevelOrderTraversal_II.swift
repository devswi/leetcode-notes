import Foundation

/// https://leetcode-cn.com/problems/binary-tree-level-order-traversal-ii/
public func levelOrderBottom(_ root: TreeNode?) -> [[Int]] {
    guard root != nil else { return [] }
    var result: [[Int]] = []
    var tmp: [TreeNode?] = [root]
    while !tmp.isEmpty {
        var val: [Int] = []
        var subtrees: [TreeNode?] = []
        for i in stride(from: 0, to: tmp.count, by: 1) {
            if let node = tmp[i] {
                val.append(node.val)
                if let left = node.left {
                    subtrees.append(left)
                }
                if let right = node.right {
                    subtrees.append(right)
                }
            }
        }
        result.insert(val, at: 0)
        tmp = subtrees
    }
    return result
}
