// 1106. Parsing A Boolean Expression
// https://leetcode.cn/problems/parsing-a-boolean-expression/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for ch in expression.chars() {
            match ch {
                ',' => continue,
                ')' => {
                    let mut count_t = 0;
                    let mut count_f = 0;
                    while stack.last() != Some(&'(') {
                        if let Some(k) = stack.pop() {
                            if k == 't' {
                                count_t += 1;
                            } else if k == 'f' {
                                count_f += 1;
                            }
                        }
                    }
                    stack.pop();
                    if let Some(k) = stack.pop() {
                        match k {
                            '!' => stack.push(if count_f == 1 { 't' } else { 'f' }),
                            '&' => stack.push(if count_f == 0 { 't' } else { 'f' }),
                            '|' => stack.push(if count_t != 0 { 't' } else { 'f' }),
                            _ => {}
                        }
                    }
                }
                _ => stack.push(ch),
            }
        }
        stack[0] == 't'
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1106() {
        assert!(!Solution::parse_bool_expr("&(|(f))".to_string()),);
        assert!(Solution::parse_bool_expr("|(f,f,f,t)".to_string()),);
        assert!(!Solution::parse_bool_expr("&(t,f)".to_string()),);
        assert!(!Solution::parse_bool_expr("|(&(t,f,t),!(t))".to_string()),);
        assert!(Solution::parse_bool_expr("!(&(f,t))".to_string()),);
        assert!(Solution::parse_bool_expr("|(t,t,t)".to_string()),);
    }
}
