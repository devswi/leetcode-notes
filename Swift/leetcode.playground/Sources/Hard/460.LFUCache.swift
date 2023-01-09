import Foundation

public class LFUCache {
    
    class Node {
        var frequency: Int
        var value: Int
        var key: Int
        
        init(key: Int, value: Int) {
            self.key = key
            self.value = value
            self.frequency = 1
        }
    }
    
    let capacity: Int

    // Key: 存储 node 的键
    // Value: Node 对象
    var cache: [Int: Node] = [:]
    
    // 结构
    // key: 出现次数
    // value: 出现 key 值次数的 node [node1, node2, ..., node(n)] 为队列，队首的值为最早操作过的 node，队尾为最新操作的
    // 移除相同频次的 node 实际变为移除队首 node
    var frequencyDict: [Int: [Node]] = [:]
    
    var min = 0
    
    public init(_ capacity: Int) {
        self.capacity = capacity
    }
    
    public func get(_ key: Int) -> Int {
        guard let node = cache[key] else { return -1 }
        updateFrequency(node: node)
        return node.value
    }
    
    public func put(_ key: Int, _ value: Int) {
        if capacity == 0 { return }
        guard let node = cache[key] else {
            if cache.count == capacity {
                removeNode() // 移除出现 min 次的 node 数组中第一个 node 的缓存
            }
            let node = Node(key: key, value: value)
            add(node: node) // 添加新的节点
            return
        }
        
        node.value = value
        updateFrequency(node: node)
    }
    
    private func updateFrequency(node: Node) {
        let freq = node.frequency
        guard var nodes = frequencyDict[freq] else { return }
        nodes.removeAll(where: { $0.key == node.key })
        frequencyDict[freq] = nodes // 更新旧频次中的 node
        
        if freq == min && nodes.isEmpty { min = freq + 1 }
        
        node.frequency += 1
        var new = frequencyDict[freq + 1] ?? []
        
        new.append(node) // 当前节点放入队尾
        frequencyDict[freq + 1] = new // 移动到新的频次
    }
    
    private func add(node: Node) {
        let freq = node.frequency
        var nodes = frequencyDict[freq] ?? []
        nodes.append(node)
        frequencyDict[freq] = nodes // 更新频次 dict
        
        min = node.frequency // 更新最小频次
        cache[node.key] = node
    }
    
    private func removeNode() {
        var nodes = frequencyDict[min] ?? []
        let node = nodes[0]
        nodes.removeAll(where: { $0.key == node.key })
        frequencyDict[min] = nodes // 更新频次 dict
        cache[node.key] = nil // 移除 cache 中的 node
    }
    
}
