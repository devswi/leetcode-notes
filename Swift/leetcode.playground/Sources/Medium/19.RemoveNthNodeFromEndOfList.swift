import Foundation

/// https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/
public func removeNthFromEnd(_ head: ListNode?, _ n: Int) -> ListNode? {
    var fast = head, slow: ListNode? = nil, n: Int = n
    while let nd = fast {
        if n != 0 {
            fast = nd.next
            n -= 1
            continue
        }
        fast = fast?.next
        slow = slow == nil ? head : slow?.next
    }
    if slow == nil {
        return head?.next
    } else {
        slow?.next = slow?.next?.next
        return head
    }
}
