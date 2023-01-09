import Foundation

// https://leetcode-cn.com/problems/swap-nodes-in-pairs/
public func swapPairs(_ head: ListNode?) -> ListNode? {
    guard let next = head?.next else { return head }
    let t = next.next
    head?.next = t
    next.next = head
    head?.next = swapPairs(t)
    return next
}
