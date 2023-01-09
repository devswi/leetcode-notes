import Foundation

/// https://leetcode-cn.com/problems/reverse-linked-list/
public func reverseList(_ head: ListNode?) -> ListNode? {
    // 迭代
//    var pre: ListNode? = nil
//    var current = head
//    var temp: ListNode? = nil
//    while current != nil {
//        temp = current?.next
//        current?.next = pre
//        pre = current
//        current = temp
//    }
//    return pre
    
    // 递归
    guard head != nil && head?.next != nil else { return head }
    let current = reverseList(head?.next)
    head?.next?.next = head
    head?.next = nil
    return current
}
