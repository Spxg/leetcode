pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
        let mut stack = Vec::new();
        for char in s.chars() {
            if let Some(v) = stack.last() {
                if map.get(v).eq(&Some(&char)) {
                    stack.pop();
                } else {
                    stack.push(char);
                }
            } else {
                stack.push(char);
            }
        }
        stack.is_empty()
    }
}

impl super::Solution for Solution {
    fn is_valid(s: String) -> bool {
        Self::is_valid(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
