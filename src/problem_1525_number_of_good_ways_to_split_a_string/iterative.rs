pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut result = 0;
        let mut right = HashMap::new();
        for ch in s.chars() {
            *right.entry(ch).or_insert(0) += 1;
        }

        let mut left = HashSet::new();
        for ch in s.chars() {
            left.insert(ch);
            let count = right.get(&ch).copied().unwrap();
            if count == 1 {
                right.remove(&ch);
            } else {
                *right.get_mut(&ch).unwrap() = count - 1;
            }
            result += i32::from(left.len() == right.len());
        }

        result
    }
}

impl super::Solution for Solution {
    fn num_splits(s: String) -> i32 {
        Self::num_splits(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
