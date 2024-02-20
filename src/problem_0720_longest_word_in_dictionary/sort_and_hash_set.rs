pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut words = words;
        let mut result = String::new();
        let mut set = HashSet::from([String::new()]);

        words.sort_unstable();

        for word in words {
            let (prefix, _) = word.split_at(word.len() - 1);
            if set.contains(prefix) {
                set.insert(word.clone());
                if word.len() > result.len() {
                    result = word;
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn longest_word(words: Vec<String>) -> String {
        Self::longest_word(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
