pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();

        let mut patterns = pattern.chars();
        let mut words = s.split_whitespace();

        loop {
            match (patterns.next(), words.next()) {
                (Some(pattern), Some(word)) => {
                    if word != *map1.entry(pattern).or_insert(word)
                        || pattern != *map2.entry(word).or_insert(pattern)
                    {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

impl super::Solution for Solution {
    fn word_pattern(pattern: String, s: String) -> bool {
        Self::word_pattern(pattern, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
