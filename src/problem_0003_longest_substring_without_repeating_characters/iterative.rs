pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut result = 0;

        for (prev, v1) in bytes.iter().enumerate() {
            let mut next = prev + 1;
            let mut set = HashSet::new();
            set.insert(*v1);

            while next < bytes.len() {
                if !set.insert(bytes[next]) {
                    break;
                }
                next += 1;
            }

            result = result.max(next - prev);
        }

        result as _
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
