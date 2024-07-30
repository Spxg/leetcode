pub struct Solution;

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }

        let mut power = power;
        let mut tokens = tokens;
        tokens.sort_unstable();

        let mut result = 0;
        let mut score = 0;

        let mut left = 0;
        let mut right = tokens.len() - 1;

        while left <= right {
            if tokens[left] <= power {
                power -= tokens[left];
                score += 1;
                left += 1;
                result = result.max(score);
            } else if score > 0 {
                power += tokens[right];
                score -= 1;
                right -= 1;
            } else {
                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        Self::bag_of_tokens_score(tokens, power)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
