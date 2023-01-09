import Foundation

/// https://leetcode-cn.com/problems/rotate-list/
public func rotateRight(_ head: ListNode?, _ k: Int) -> ListNode? {
    guard head != nil && head?.next != nil else {
        return head
    }
    var total = 1
    var tail = head
    while tail?.next != nil {
        total += 1
        tail = tail?.next
    } // 获取原链表长度
    tail?.next = head // 建立环

    for _ in stride(from: 0, through: total - k % total - 1, by: 1) {
        tail = tail?.next
    }
    
    let new = tail?.next
    tail?.next = nil // 断开环
    
    return new
    /// 暴力法: 递归，每次处理一个节点，递归栈会炸
//    if k == 0 { return head }
//    var current = head
//    while current?.next?.next != nil {
//        current = current?.next
//    }
//    let last = current?.next
//    last?.next = head
//    current?.next = nil
//    return rotateRight(last ?? head, k - 1)
}
