// 1678. Goal Parser Interpretation
// https://leetcode.cn/problems/goal-parser-interpretation/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut res = "".to_string();
        let command = command.as_str();
        let mut i = 0;
        while i < command.len() {
            if command[i..i + 1] == *"G" {
                res.push('G');
                i += 1;
            } else if command[i..=i + 1] == *"()" {
                res.push('o');
                i += 2
            } else if command[i..=i + 3] == *"(al)" {
                res.push_str("al");
                i += 4;
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
