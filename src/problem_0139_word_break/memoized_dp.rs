pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn helper(s: &str, offset: usize, set: &HashSet<String>, cache: &mut [bool]) -> bool {
            if s.is_empty() {
                return true;
            }

            for idx in 1..=s.len() {
                if set.contains(&s[0..idx]) {
                    if cache[idx + offset] && helper(&s[idx..], idx + offset, set, cache) {
                        return true;
                    }

                    cache[idx + offset] = false;
                }
            }

            false
        }
        let word_dict: HashSet<String> = word_dict.into_iter().collect();
        helper(&s, 0, &word_dict, &mut vec![true; s.len() + 1])
    }
}

impl super::Solution for Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> bool {
        Self::word_break(s, word_dict)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
