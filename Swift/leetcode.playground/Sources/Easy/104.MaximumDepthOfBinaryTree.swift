import Foundation

/// https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/
public func maxDepth(_ root: TreeNode?) -> Int {
    return root == nil ? 0 : (max(maxDepth(root?.left), maxDepth(root?.right)) + 1)
}
