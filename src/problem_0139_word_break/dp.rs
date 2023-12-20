pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_dict = word_dict.into_iter().collect::<HashSet<_>>();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for idx1 in 1..=s.len() {
            for idx2 in (0..idx1).rev() {
                if dp[idx2] && word_dict.contains(&s[idx2..idx1]) {
                    dp[idx1] = true;
                    break;
                }
            }
        }

        dp[s.len()]
    }
}

impl super::Solution for Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> bool {
        Self::word_break(s, word_dict)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
