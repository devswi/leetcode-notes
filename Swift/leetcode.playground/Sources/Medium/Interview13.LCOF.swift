import Foundation

/// https://leetcode-cn.com/problems/ji-qi-ren-de-yun-dong-fan-wei-lcof/
public func movingCount(_ m: Int, _ n: Int, _ k: Int) -> Int {
    // 记录是否走过
    var router: [(x: Int, y: Int)] = []
    
    // 递归思路构造二叉树
    //     0
    //   1   2
    //  3   4
    // TODO: 待优化
    func recursion(_ startX: Int, _ startY: Int) -> Int {
        let total = getAmount(startX) + getAmount(startY)
        let walked = router.contains(where: { $0.x == startX && $0.y == startY })
        if startX >= m || startY >= n || total > k || walked  { return 0 }
        let right = recursion(startX + 1, startY)
        let bottom = recursion(startX, startY + 1)
        let result = right + bottom
        if !walked {
            router.append((x: startX, y: startY))
        }
        return result + (walked ? 0 : 1)
    }

    
    return recursion(0, 0)
}

func getAmount(_ num: Int) -> Int {
    var res = 0, num = num
    while num > 0 {
        res += num % 10
        num = num / 10
    }
    return res
}
