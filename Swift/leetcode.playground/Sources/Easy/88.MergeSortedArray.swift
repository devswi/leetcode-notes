import Foundation

/// https://leetcode-cn.com/problems/merge-sorted-array/
public func merge(_ nums1: inout [Int], _ m: Int, _ nums2: [Int], _ n: Int) {
    var p1 = m-1, p2 = n-1, k = m + n - 1
    
    while p1 >= 0 , p2 >= 0 {
        if nums1[p1] > nums2[p2] {
            nums1[k] = nums1[p1]
            p1 -= 1
        }
        else {
            nums1[k] = nums2[p2]
            p2 -= 1
        }
        k -= 1
    }
    
    while p2 >= 0 {
        nums1[p2] = nums2[p2]
        p2 -= 1
    }
}
