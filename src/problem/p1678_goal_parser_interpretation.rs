// 1678. Goal Parser Interpretation
// https://leetcode.cn/problems/goal-parser-interpretation/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut stack = vec![];
        for c in command.chars() {
            match c {
                ')' => {
                    if stack.last() == Some(&'(') {
                        stack.pop();
                        stack.push('o');
                    } else {
                        stack.pop(); // l
                        stack.pop(); // a
                        stack.pop(); // (
                        stack.push('a');
                        stack.push('l')
                    }
                }
                _ => stack.push(c),
            }
        }
        stack.into_iter().collect()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1678() {
        assert_eq!(
            Solution::interpret("G()(al)".to_string()),
            "Goal".to_string()
        );
        assert_eq!(
            Solution::interpret("G()()()()(al)".to_string()),
            "Gooooal".to_string()
        );
        assert_eq!(
            Solution::interpret("(al)G(al)()()G".to_string()),
            "alGalooG".to_string()
        );
    }
}
