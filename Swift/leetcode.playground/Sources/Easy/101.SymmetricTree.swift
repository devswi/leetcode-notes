import Foundation

/// https://leetcode-cn.com/problems/symmetric-tree/
public func isSymmetric(_ root: TreeNode?) -> Bool {
    return iteration(root?.left, root?.right)
}

// 递归
func recursion(_ p: TreeNode?, _ q: TreeNode?) -> Bool {
    if p == nil && q == nil { return true }
    return p?.val == q?.val && recursion(p?.left, q?.right) && recursion(p?.right, q?.left)
}

// 迭代
func iteration(_ p: TreeNode?, _ q: TreeNode?) -> Bool {
    var queue: [TreeNode?] = [p, q]
    while !queue.isEmpty {
        let q = queue.popLast() ?? nil
        let p = queue.popLast() ?? nil
        if p == nil && q == nil { continue }
        if p?.val != q?.val { return false }
        queue.append(q?.left)
        queue.append(p?.right)
        queue.append(q?.right)
        queue.append(p?.left)
    }
    return true
}
