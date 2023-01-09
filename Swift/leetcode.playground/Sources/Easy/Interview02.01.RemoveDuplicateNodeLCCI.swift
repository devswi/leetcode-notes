import Foundation

/// https://leetcode-cn.com/problems/remove-duplicate-node-lcci/
public func removeDuplicateNodes(_ head: ListNode?) -> ListNode? {
    guard let head = head else { return nil }
    var alreadyExist: Set<Int> = [head.val] // 借助集合类型，判断元素是否出现
    var pre = head
    while let next = pre.next {
        let val = next.val
        if alreadyExist.contains(val) { // 重复，则移除该节点
            pre.next = pre.next?.next
        } else { // 不重复，存储并继续向后遍历链表
            alreadyExist.insert(val)
            pre = next
        }
    }
    return head
}
