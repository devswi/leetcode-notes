// 1773. Count Items Matching a Rule
// https://leetcode.cn/problems/count-items-matching-a-rule/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut ans = 0;
        let idx = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            _ => 2,
        };
        for item in items {
            ans += if item[idx] == rule_value { 1 } else { 0 }
        }
        ans
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1773() {
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "silver".to_string()
                    ]
                ],
                "color".to_string(),
                "silver".to_string()
            ),
            1
        );
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "silver".to_string()
                    ]
                ],
                "type".to_string(),
                "phone".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "silver".to_string()
                    ]
                ],
                "color".to_string(),
                "ii".to_string()
            ),
            0
        );
    }
}
