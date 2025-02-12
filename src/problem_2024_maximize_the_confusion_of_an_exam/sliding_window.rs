pub struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut result = 0;
        let answer = answer_key.into_bytes();

        let (mut left, mut right) = (0, 0);
        let mut count_t = 0;
        let mut count_f = 0;

        while right < answer.len() {
            if answer[right] == b'T' {
                count_t += 1;
            } else {
                count_f += 1;
            }
            if count_t > k && count_f > k {
                if answer[left] == b'T' {
                    count_t -= 1;
                } else {
                    count_f -= 1;
                }
                left += 1;
            }
            result = result.max(count_t + count_f);
            right += 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        Self::max_consecutive_answers(answer_key, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
