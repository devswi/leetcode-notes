import Foundation

/// https://leetcode-cn.com/problems/balanced-binary-tree/
public func isBalanced(_ root: TreeNode?) -> Bool {
    if root == nil { return true }
    if abs(length(root?.left) - length(root?.right)) > 1 { return false }
    return isBalanced(root?.left) && isBalanced(root?.right)
}

func length(_ root: TreeNode?) -> Int {
    return root == nil ? 0 : (max(length(root?.left), length(root?.right)) + 1)
}
