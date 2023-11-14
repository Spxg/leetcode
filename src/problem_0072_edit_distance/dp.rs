pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let m = word1.len();
        let n = word2.len();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for x in 0..=m {
            for y in 0..=n {
                dp[x][y] = if x == 0 {
                    y
                } else if y == 0 {
                    x
                } else if word1[x - 1] == word2[y - 1] {
                    dp[x - 1][y - 1]
                } else {
                    dp[x - 1][y].min(dp[x][y - 1]).min(dp[x - 1][y - 1]) + 1
                }
            }
        }

        dp[m][n] as _
    }
}

impl super::Solution for Solution {
    fn min_distance(word1: String, word2: String) -> i32 {
        Self::min_distance(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
