pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut result = 0;

        let mut map = HashMap::with_capacity(arr.len());
        for num in arr {
            let count = map.get(&(num - difference)).map_or(1, |count| count + 1);
            result = result.max(count);
            map.insert(num, count);
        }

        result
    }
}

impl super::Solution for Solution {
    fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        Self::longest_subsequence(arr, difference)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
