pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut position = -1;
        let mut result = 0;

        for (idx, ele) in s.chars().enumerate() {
            if let Some(i) = map.insert(ele, idx) {
                position = position.max(i as i32);
            }
            result = result.max(idx as i32 - position);
        }
        result
    }
}

impl super::Solution for Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        Self::length_of_longest_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
