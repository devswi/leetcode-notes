import Foundation

/// https://leetcode-cn.com/problems/convert-sorted-list-to-binary-search-tree/
public func sortedListToBST(_ head: ListNode?) -> TreeNode? {
    guard let head = head else { return nil }
    guard head.next != nil else { return TreeNode(head.val) }
    var fast: ListNode? = head.next, slow: ListNode? = head, pre: ListNode? = head
    while fast != nil {
        fast = fast?.next?.next
        pre = slow
        slow = slow?.next
    }
    guard let mid = slow else { return nil }
    pre?.next = nil
    let node = TreeNode(mid.val)
    node.left = sortedListToBST(head)
    node.right = sortedListToBST(slow?.next)
    return node
}
