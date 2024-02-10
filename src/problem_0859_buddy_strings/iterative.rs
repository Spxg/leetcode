pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let set = s.bytes().collect::<HashSet<_>>();
        let swap = s
            .bytes()
            .zip(goal.bytes())
            .filter_map(|(a, b)| (a != b).then_some((a, b)))
            .collect::<Vec<_>>();

        match swap.as_slice() {
            [(v1, v2), (v3, v4)] => v1 == v4 && v2 == v3,
            [] => set.len() != s.len(),
            _ => false,
        }
    }
}

impl super::Solution for Solution {
    fn buddy_strings(s: String, goal: String) -> bool {
        Self::buddy_strings(s, goal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
