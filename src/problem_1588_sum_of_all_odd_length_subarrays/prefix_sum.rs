pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prefix_sum = HashMap::new();
        prefix_sum.insert(0, 0);
        let mut sum = 0;
        for (pos, val) in (1..).zip(arr) {
            sum += val;
            prefix_sum.insert(pos, sum);
            for x in (i32::from(pos % 2 == 0)..pos).step_by(2) {
                result += sum - prefix_sum[&x];
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        Self::sum_odd_length_subarrays(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
