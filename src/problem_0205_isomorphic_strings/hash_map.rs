pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        let mut set = HashSet::new();

        for (x, y) in s.chars().zip(t.chars()) {
            if map
                .insert(x, y)
                .map_or_else(|| !set.insert(y), |exist| exist != y)
            {
                return false;
            }
        }
        true
    }
}

impl super::Solution for Solution {
    fn is_isomorphic(s: String, t: String) -> bool {
        Self::is_isomorphic(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
