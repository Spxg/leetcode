pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned = banned.into_iter().collect::<HashSet<_>>();
        let mut map = HashMap::new();

        for s in paragraph
            .split(&[' ', '!', '?', '\'', ',', ';', '.'])
            .map(str::to_lowercase)
            .filter(|x| !x.is_empty())
        {
            *map.entry(s).or_insert(0) += 1;
        }

        let mut max_count = 0;
        let mut result = String::new();

        for (s, count) in map {
            if count > max_count && !banned.contains(&s) {
                result = s;
                max_count = count.max(max_count);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        Self::most_common_word(paragraph, banned)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
