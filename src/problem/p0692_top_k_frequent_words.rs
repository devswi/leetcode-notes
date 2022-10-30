// 0692. Top K Frequent Words
// https://leetcode.cn/problems/top-k-frequent-words/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        use std::collections::HashMap;
        let mut dict = words
            .iter()
            .fold(&mut HashMap::new(), |m, w| {
                *m.entry(w).or_insert(0) += 1;
                m
            })
            .iter()
            .map(|(&k, &v)| (k, v))
            .collect::<Vec<_>>();
        dict.sort_by(|a, b| {
            if b.1 == a.1 {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        let mut answer = Vec::new();
        for (i, val) in dict.iter().enumerate() {
            answer.push(val.0.clone());
            if i as i32 == k - 1 {
                break;
            }
        }
        answer
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0692() {
        assert_eq!(
            Solution::top_k_frequent(
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "coding".to_string()
                ],
                2
            ),
            vec!["i".to_string(), "love".to_string()]
        );
    }
}
