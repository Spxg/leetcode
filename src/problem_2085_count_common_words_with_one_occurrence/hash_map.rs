pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut result = 0;

        let mut map1 = HashMap::with_capacity(words1.len());
        let mut map2 = HashMap::with_capacity(words2.len());

        for word in &words1 {
            *map1.entry(word).or_insert(0) += 1;
        }
        for word in &words2 {
            *map2.entry(word).or_insert(0) += 1;
        }

        for word in &words1 {
            result += i32::from(map1.get(word) == Some(&1) && map2.get(word) == Some(&1));
        }
        result
    }
}

impl super::Solution for Solution {
    fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        Self::count_words(words1, words2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
