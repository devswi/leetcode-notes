import Foundation

/// https://leetcode-cn.com/problems/split-linked-list-in-parts/
public func splitListToParts(_ root: ListNode?, _ k: Int) -> [ListNode?] {
    var ans = [ListNode?](repeating: nil, count: k)
    // 计算节点个数
    var total = 0
    var current = root
    while current != nil {
        total += 1
        current = current?.next
    }
    // 计算每个分割部分的长度
    var length = [Int](repeating: total / k, count: k)
    let remainder = total % k
    for i in stride(from: 0, to: remainder, by: 1) {
        length[i] += 1
    }
    
    // 再次遍历，截取
    var head = root
    var s = root
    var pre = root
    for i in stride(from: 0, to: k, by: 1) {
        var l = length[i]
        while l != 0 {
            pre = s
            s = s?.next
            l -= 1
        }
        pre?.next = nil
        ans[i] = head
        head = s
        pre = s
    }
    
    return ans
}
