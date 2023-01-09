import Foundation

/// https://leetcode-cn.com/problems/as-far-from-land-as-possible/
public func maxDistance(_ grid: [[Int]]) -> Int {
    var grid = grid
    var lands: [(x: Int, y: Int)] = []
    var hasOcean = false
    
    let n = grid.count
    
    for x in stride(from: 0, to: n, by: 1) {
        for y in stride(from: 0, to: n, by: 1) {
            let item = grid[x][y]
            if item == 1 {
                lands.append((x: x, y: y))
            } else {
                hasOcean = true
            }
        }
    }

    guard !lands.isEmpty && hasOcean else { // 没有陆地或着没有海洋
        return -1
    }
    
    // 上，下，左，右
    let dX = [0, 0, -1, 1]
    let dY = [-1, 1, 0, 0]
    
    var land = lands[0]
    
    while !lands.isEmpty {
        land = lands.removeFirst()
        let x = land.x, y = land.y
        
        for i in stride(from: 0, to: 4, by: 1) {
            let newX = x + dX[i]
            let newY = y + dY[i]
            
            if newX < 0 || newX >= n || newY < 0 || newY >= n || grid[newX][newY] != 0 {
                continue
            }
            
            grid[newX][newY] = grid[x][y] + 1
            lands.append((x: newX, y: newY))
        }
    }
    
    return grid[land.x][land.y] - 1
}
