pub struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1
            .iter()
            .flat_map(|s| s.bytes())
            .eq(word2.iter().flat_map(|s| s.bytes()))
    }
}

impl super::Solution for Solution {
    fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        Self::array_strings_are_equal(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
