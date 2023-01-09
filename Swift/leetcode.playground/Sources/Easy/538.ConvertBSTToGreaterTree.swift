import Foundation

/// https://leetcode-cn.com/problems/convert-bst-to-greater-tree/
/// [二叉搜索树](https://zh.wikipedia.org/wiki/%E4%BA%8C%E5%85%83%E6%90%9C%E5%B0%8B%E6%A8%B9)

var sum = 0

@discardableResult
public func convertBST(_ root: TreeNode?) -> TreeNode? {
    guard let root = root else { return nil }
    convertBST(root.right)
    sum += root.val
    root.val = sum
    convertBST(root.left)
    return root
}
