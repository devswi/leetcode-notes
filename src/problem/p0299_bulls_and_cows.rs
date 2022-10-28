// 0299. Bulls and Cows
// https://leetcode.cn/problems/bulls-and-cows/

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut count_s = vec![0; 10];
        let mut count_g = vec![0; 10];

        for (s, g) in secret.bytes().zip(guess.bytes()) {
            if s == g {
                bulls += 1;
            } else {
                count_s[(s - b'0') as usize] += 1;
                count_g[(g - b'0') as usize] += 1;
            }
        }

        for i in 0..10 {
            cows += count_s[i].min(count_g[i]);
        }

        format!("{}A{}B", bulls, cows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0299() {
        assert_eq!(
            Solution::get_hint("1807".to_string(), "7810".to_string()),
            "1A3B".to_string()
        );
        assert_eq!(
            Solution::get_hint("1123".to_string(), "0111".to_string()),
            "1A1B".to_string()
        );
    }
}
