pub struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map = HashMap::with_capacity(s1.len());
        let mut cache = VecDeque::with_capacity(s1.len());
        s1.bytes().for_each(|x| *map.entry(x).or_insert(0) += 1);

        for ch in s2.bytes() {
            if let Some(val) = map.get_mut(&ch) {
                cache.push_back(ch);
                *val -= 1;
                if *val == 0 {
                    map.remove(&ch);
                    if map.is_empty() {
                        return true;
                    }
                }
            } else {
                while let Some(val) = cache.pop_front() {
                    if val == ch {
                        cache.push_back(ch);
                        break;
                    }
                    *map.entry(val).or_insert(0) += 1;
                }
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn check_inclusion(s1: String, s2: String) -> bool {
        Self::check_inclusion(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
