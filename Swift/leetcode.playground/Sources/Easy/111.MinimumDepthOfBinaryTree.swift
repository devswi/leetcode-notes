import Foundation

/// https://leetcode-cn.com/problems/minimum-depth-of-binary-tree/
public func minDepth(_ root: TreeNode?) -> Int {
    if root == nil { return 0 }
    let l = minDepth(root?.left)
    let r = minDepth(root?.right)
    return (root?.left == nil || root?.right == nil) ? (l + r + 1) : (min(l, r) + 1)
}
