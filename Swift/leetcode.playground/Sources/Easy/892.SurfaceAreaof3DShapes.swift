import Foundation

public func surfaceArea(_ grid: [[Int]]) -> Int {
    let length = grid.count
    
    var total = 0
    for i in stride(from: 0, to: length, by: 1) {
        for j in stride(from: 0, to: length, by: 1) {
            let n = grid[i][j]
            if n == 0 { continue }
            total += n * 4 + 2 // 6 * n - (n - 1) * 2
            if i + 1 < length { // next in x
                let left = grid[i + 1][j]
                total -= min(left, n) * 2
            }
            if j + 1 < length { // next in y
                let bottom = grid[i][j + 1]
                total -= min(bottom, n) * 2
            }
        }
    }
    return total
}
