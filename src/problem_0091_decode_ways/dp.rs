pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes[0] - b'0' == 0 {
            return 0;
        }
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;

        for idx in 1..=s.len() {
            dp[idx] = if bytes[idx - 1] == b'0' {
                0
            } else {
                dp[idx - 1]
            };
            if idx > 1
                && (bytes[idx - 2] == b'1' || bytes[idx - 2] == b'2' && bytes[idx - 1] <= b'6')
            {
                dp[idx] += dp[idx - 2];
            }
        }
        dp[s.len()]
    }
}

impl super::Solution for Solution {
    fn num_decodings(s: String) -> i32 {
        Self::num_decodings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
