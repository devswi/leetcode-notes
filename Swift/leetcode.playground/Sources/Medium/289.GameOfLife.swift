import Foundation

/// https://leetcode-cn.com/problems/game-of-life/
public func gameOfLife(_ board: inout [[Int]]) {
    var changes: [(x: Int, y: Int)] = [] // 关键思路，不能直接修改原数组
    // 需要满足下一个时间段同时发生，因此需要数据用来记录将要改变的细胞
    
    let m = board.count
    if m == 0 { return }
    let n = board[0].count
    
    // 左上，上，右上，右，右下，下，左下，左
    let dx = [-1, 0, 1, 1, 1, 0, -1, -1]
    let dy = [-1, -1, -1, 0, 1, 1, 1, 0]

    for i in stride(from: 0, to: m, by: 1) {
        for j in stride(from: 0, to: n, by: 1) {
            let cell = board[i][j]
            var live = 0
            for index in 0..<8 {
                let x = i + dx[index]
                let y = j + dy[index]
                if x < 0 || x >= m || y < 0 || y >= n || board[x][y] == 0 { continue }
                live += 1
            }
            
            if (live < 2 && cell == 1) || (live > 3 && cell == 1) || (live == 3 && cell == 0) {
                changes.append((x: i, y: j))
            }
        }
    }
    
    // 遍历完原始数组之后，获取需要修改状态的坐标，遍历，修改原数组
    for change in changes {
        board[change.x][change.y] = board[change.x][change.y] == 0 ? 1 : 0
    }
}
