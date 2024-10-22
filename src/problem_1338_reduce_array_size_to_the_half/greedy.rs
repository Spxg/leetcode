pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut result = 0;

        let length = arr.len() as i32;
        let mut counts = vec![0; 100_000];
        for num in arr {
            counts[num as usize - 1] += 1;
        }
        let mut counts = counts.into_iter().collect::<BinaryHeap<_>>();

        let mut totol = 0;
        while totol < length / 2 {
            result += 1;
            totol += counts.pop().unwrap();
        }

        result
    }
}

impl super::Solution for Solution {
    fn min_set_size(arr: Vec<i32>) -> i32 {
        Self::min_set_size(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
