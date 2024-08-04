pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map = HashMap::new();
        for s in s1.split(' ') {
            *map.entry(s).or_insert(0) += 1;
        }
        for s in s2.split(' ') {
            *map.entry(s).or_insert(0) += 1;
        }
        map.into_iter()
            .filter(|&(_, count)| count == 1)
            .map(|(s, _)| s.to_string())
            .collect()
    }
}

impl super::Solution for Solution {
    fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        Self::uncommon_from_sentences(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
