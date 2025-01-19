pub struct Solution;

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        fn helper(word: String) -> i32 {
            word.bytes()
                .fold(0, |acc, x| acc * 10 + i32::from(x - b'a'))
        }
        helper(first_word) + helper(second_word) == helper(target_word)
    }
}

impl super::Solution for Solution {
    fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        Self::is_sum_equal(first_word, second_word, target_word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
