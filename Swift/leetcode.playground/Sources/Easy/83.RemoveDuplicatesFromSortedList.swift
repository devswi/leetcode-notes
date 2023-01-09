import Foundation

/// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/
public func deleteDuplicates(_ head: ListNode?) -> ListNode? {
    var node = head
    while let nd = node {
        if nd.val == nd.next?.val {
            nd.next = nd.next?.next
        } else {
            node = nd.next
        }
    }
    return head
}
