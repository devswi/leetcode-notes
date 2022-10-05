// 811. Subdomain Visit Count
// https://leetcode.cn/problems/subdomain-visit-count/

use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut res: HashMap<String, i32> = HashMap::new();
        for domains in cpdomains {
            let domain_with_count: Vec<&str> = domains.split_whitespace().collect();
            let count: i32 = domain_with_count[0].parse().unwrap();
            let mut domain = domain_with_count[1];
            while !domain.is_empty() {
                res.entry(domain.to_string())
                    .and_modify(|x| *x += count)
                    .or_insert(count);
                domain = domain.find('.').map_or("", |idx| &domain[idx + 1..]);
            }
        }
        res.iter().map(|(x, y)| format!("{} {}", y, x)).collect()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0811() {
        assert_eq!(
            Solution::subdomain_visits(vec![String::from("9001 discuss.leetcode.com")]),
            vec![
                String::from("9001 leetcode.com"),
                String::from("9001 discuss.leetcode.com"),
                String::from("9001 com")
            ]
        );
        assert_eq!(
            Solution::subdomain_visits(vec![
                String::from("900 google.mail.com"),
                String::from("50 yahoo.com"),
                String::from("1 intel.mail.com"),
                String::from("5 wiki.org")
            ]),
            vec![
                String::from("901 mail.com"),
                String::from("50 yahoo.com"),
                String::from("900 google.mail.com"),
                String::from("5 wiki.org"),
                String::from("5 org"),
                String::from("1 intel.mail.com"),
                String::from("951 com")
            ]
        );
    }
}
