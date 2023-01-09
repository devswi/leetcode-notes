import Foundation

/// https://leetcode-cn.com/problems/add-binary/
public func addBinary(_ a: String, _ b: String) -> String {
    // a,b 字符串
    // 倒序遍历，同为 "1" 产生进位
    // 不同为 "1" 不进位，两数之和 return
    var extra = 0
    var ans = ""
    var aOffset = 1
    var bOffset = 1
    while aOffset <= a.count || bOffset <= b.count {
        if aOffset <= a.count {
            extra += a[a.index(a.endIndex, offsetBy: -aOffset)] == "0" ? 0 : 1
        }
        if bOffset <= b.count {
            extra += b[b.index(b.endIndex, offsetBy: -bOffset)] == "0" ? 0 : 1
        }
        ans += "\(extra % 2)"
        extra /= 2
        aOffset += 1
        bOffset += 1
    }
    if extra == 0 {
        return String(ans.reversed())
    } else {
        return "1" + String(ans.reversed())
    }
}
