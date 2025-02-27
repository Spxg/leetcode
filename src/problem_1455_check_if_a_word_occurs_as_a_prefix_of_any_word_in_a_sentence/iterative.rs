pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(' ')
            .position(|x| x.starts_with(&search_word))
            .map_or(-1, |x| x as i32 + 1)
    }
}

impl super::Solution for Solution {
    fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        Self::is_prefix_of_word(sentence, search_word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
