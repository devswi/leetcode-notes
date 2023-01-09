import Foundation

/// https://leetcode-cn.com/problems/add-two-numbers/
public func addTwoNumbers(_ l1: ListNode?, _ l2: ListNode?) -> ListNode? {
    let result: ListNode? = ListNode(0)
    var l1 = l1, l2 = l2, current = result
    var index = 0 // 进位
    while l1 != nil || l2 != nil {
        let left = l1?.val ?? 0
        let right = l2?.val ?? 0
        let r = left + right + index
        index = r / 10
        current?.next = ListNode(r % 10)
        current = current?.next
        l1 = l1?.next
        l2 = l2?.next
    }
    if index == 1 {
        current?.next = ListNode(1)
    }
    return result?.next
}
