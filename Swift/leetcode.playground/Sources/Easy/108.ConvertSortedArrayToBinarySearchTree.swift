import Foundation

/// https://leetcode-cn.com/problems/convert-sorted-array-to-binary-search-tree/
public func sortedArrayToBST(_ nums: [Int]) -> TreeNode? {
    return half(nums, 0, nums.count - 1)
}

func half(_ nums: [Int], _ left: Int, _ right: Int) -> TreeNode? {
    if left == right {
        return TreeNode(nums[left])
    } else if left > right {
        return nil
    }
    let mid = (left + right + 1) / 2
    let node = TreeNode(nums[mid])
    node.left = half(nums, left, mid - 1)
    node.right = half(nums, mid + 1, right)
    return node
}
