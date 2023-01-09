import Foundation

/// https://leetcode-cn.com/problems/middle-of-the-linked-list/
public func middleNode(_ head: ListNode?) -> ListNode? {
    // 1. 第一次遍历统计节点个数，第二次遍历
//    var total = 0
//    var current = head
//    while current != nil {
//        total += 1
//        current = current?.next
//    }
//    var middle = (total / 2)
//    var newHead = head
//    while middle != 0 {
//        newHead = newHead?.next
//        middle -= 1
//    }
//    return newHead
    
    // 2. 快慢指针
    // !!! 对于链表的中值问题，快慢指针是最优解
    var fast = head
    var slow = head
    while fast != nil && fast?.next != nil {
        fast = fast?.next?.next
        slow = slow?.next
    }
    return slow
}
