// 0901. Online Stock Span
// https://leetcode.cn/problems/online-stock-span/

#[allow(dead_code)]
struct StockSpanner {
    prices: Vec<i32>,
}

#[allow(dead_code)]
impl StockSpanner {
    fn new() -> Self {
        StockSpanner { prices: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.prices.push(price);
        let mut res = 0;
        for v in self.prices.iter().rev() {
            if *v <= price {
                res += 1;
            } else {
                break;
            }
        }
        res
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0901() {
        let mut obj = StockSpanner::new();
        let ret: i32 = obj.next(100);
        assert_eq!(ret, 1);
        let ret: i32 = obj.next(80);
        assert_eq!(ret, 1);
        let ret: i32 = obj.next(60);
        assert_eq!(ret, 1);
        let ret: i32 = obj.next(70);
        assert_eq!(ret, 2);
        let ret: i32 = obj.next(60);
        assert_eq!(ret, 1);
        let ret: i32 = obj.next(75);
        assert_eq!(ret, 4);
        let ret: i32 = obj.next(85);
        assert_eq!(ret, 6);
    }
}
