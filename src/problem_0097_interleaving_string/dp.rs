pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());

        let mut dp = vec![false; s2.len() + 1];
        for idx1 in 0..=s1.len() {
            for idx2 in 0..=s2.len() {
                dp[idx2] = (idx1 == 0 && idx2 == 0)
                    || (idx1 > 0 && dp[idx2] && s1[idx1 - 1] == s3[idx1 + idx2 - 1])
                    || (idx2 > 0 && dp[idx2 - 1] && s2[idx2 - 1] == s3[idx1 + idx2 - 1]);
            }
        }

        dp[s2.len()]
    }
}

impl super::Solution for Solution {
    fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        Self::is_interleave(s1, s2, s3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
