import Foundation

/// https://leetcode-cn.com/problems/3sum/
public func threeSum(_ nums: [Int]) -> [[Int]] {
    var result: [[Int]] = []
    
    guard nums.count >= 3 else { return result }
    
    let nums = nums.sorted()
    
    for i in stride(from: 0, to: nums.count - 2, by: 1) {
        if i > 0 && nums[i] == nums[i - 1] { // 忽略重复的值
            continue
        }
    
        let first = nums[i], remaining = -first
        var m = i + 1, n = nums.count - 1
        
        while m < n {
            if nums[m] + nums[n] == remaining {
                result.append([first, nums[m], nums[n]])
                
                repeat {
                    m += 1
                } while nums[m] == nums[m - 1] && m < n
                
                repeat {
                    n -= 1
                } while nums[n] == nums[n + 1] && m < n
            } else if nums[m] + nums[n] < remaining {
                m += 1
            } else {
                n -= 1
            }
        }
    }
    
    return result
}
