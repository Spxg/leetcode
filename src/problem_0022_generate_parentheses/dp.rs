pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn generate(map: &mut HashMap<i32, Vec<String>>, n: i32) {
        let mut result = HashSet::new();
        for i in 1..=n / 2 {
            if map.get(&i).is_none() {
                Self::generate(map, i);
            }
            if map.get(&(n - i)).is_none() {
                Self::generate(map, n - i);
            }
            for x in &map[&i] {
                for y in &map[&(n - i)] {
                    result.insert(format!("{x}{y}"));
                    result.insert(format!("{y}{x}"));
                }
            }
        }
        for x in map.get(&(n - 1)).unwrap() {
            result.insert(format!("({x})"));
        }
        map.insert(n, result.into_iter().collect());
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut map = HashMap::from([(0, vec![String::new()])]);
        Self::generate(&mut map, n);
        map.remove(&n).unwrap()
    }
}

impl super::Solution for Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::generate_parenthesis(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
