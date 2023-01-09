import Foundation

/// https://leetcode-cn.com/problems/path-sum/
public func hasPathSum(_ root: TreeNode?, _ sum: Int) -> Bool {
    if root == nil { return false }
    let left = hasPathSum(root?.left, sum - root!.val)
    let right = hasPathSum(root?.right, sum - root!.val)
    return (root?.left == nil && root?.right == nil) ? (sum - root!.val) == 0 : (left || right)
}
