pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let map = numbers.iter().zip(1..).collect::<HashMap<_, _>>();
        for (&number, idx1) in numbers.iter().zip(1..) {
            if let Some(&idx2) = map.get(&(target - number)) {
                return vec![idx1, idx2];
            }
        }
        unreachable!()
    }
}

impl super::Solution for Solution {
    fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(numbers, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
