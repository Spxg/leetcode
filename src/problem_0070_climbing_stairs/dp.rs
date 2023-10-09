pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn helper(record: &mut HashMap<i32, i32>, n: i32) -> i32 {
            let value = record
                .remove(&n)
                .unwrap_or_else(|| helper(record, n - 1) + helper(record, n - 2));
            record.insert(n, value);
            value
        }
        let mut record = HashMap::from([(1, 1), (2, 2)]);
        helper(&mut record, n)
    }
}

impl super::Solution for Solution {
    fn climb_stairs(n: i32) -> i32 {
        Self::climb_stairs(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
