import Foundation

public class ListNode {
    public var val: Int
    public var next: ListNode?
    public init(_ val: Int) {
        self.val = val
        self.next = nil
    }
    
    public class func generate(from vals: [Int]) -> ListNode? {
        guard !vals.isEmpty else { return nil }
        let head: ListNode = ListNode(vals[0])
        var tmp: ListNode? = head
        for i in stride(from: 1, to: vals.count, by: 1) {
            tmp?.next = ListNode(vals[i])
            tmp = tmp?.next
        }
        return head
    }
    
    public func format() -> [Int] {
        var result: [Int] = []
        var node: ListNode? = self
        while let nd = node {
            result.append(nd.val)
            node = nd.next
        }
        return result
    }
}
