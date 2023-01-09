import Foundation

public func numRookCaptures(_ board: [[Character]]) -> Int {
    let length = 8
    // 遍历二维数组，返回车的位置
    var rX = 0, rY = 0
    for i in stride(from: 0, to: length, by: 1) {
        for j in stride(from: 0, to: length, by: 1) {
            if board[i][j] == "R" {
                rX = i
                rY = j
                break
            }
        }
    }
    
    // 东，南，西，北
    let moveX = [1, 0, -1, 0]
    let moveY = [0, -1, 0, 1]
    
    var capture = 0
    
    for i in stride(from: 0, to: 4, by: 1) {
        var step = 0
        repeat {
            let x = rX + step * moveX[i]
            let y = rY + step * moveY[i]
            if x < 0 || x >= length || y < 0 || y >= length || board[x][y] == "B" {
                break
            }
            if board[x][y] == "p" {
                capture += 1
                break
            }
            step += 1
        } while true
    }
    
    return capture
}
