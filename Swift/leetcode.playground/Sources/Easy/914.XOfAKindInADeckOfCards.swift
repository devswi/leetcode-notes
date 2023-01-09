import Foundation

/// https://leetcode-cn.com/problems/x-of-a-kind-in-a-deck-of-cards/
public func hasGroupsSizeX(_ deck: [Int]) -> Bool {
    var countMap: [Int: Int] = [:]
    var cards: [Int] = []
    var maxCount = 0
    
    for i in stride(from: 0, to: deck.count, by: 1) {
        let card = deck[i]
        if let count = countMap[card] {
            countMap[card] = count + 1
            maxCount = max(maxCount, count + 1)
        } else {
            countMap[card] = 1
            cards.append(card)
        }
    }
    
    if maxCount < 2 { return false }
    
    for X in stride(from: 2, through: maxCount, by: 1) {
        var result = true
        for card in cards {
            if let count = countMap[card] {
                if count < X || count % X != 0 {
                    result = false
                    break
                }
            }
        }
        if result {
            return true
        }
    }
    
    return false
}
