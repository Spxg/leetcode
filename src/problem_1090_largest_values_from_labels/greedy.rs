pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut result = 0;
        let mut pair = values.into_iter().zip(labels).collect::<Vec<_>>();
        pair.sort_unstable();

        let mut iter = pair.into_iter().rev();
        let mut map = HashMap::<i32, i32>::new();

        for _ in 0..num_wanted {
            for (num, label) in iter.by_ref() {
                if map.get(&label).is_some_and(|&count| count == use_limit) {
                    continue;
                }
                result += num;
                *map.entry(label).or_default() += 1;
                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        Self::largest_vals_from_labels(values, labels, num_wanted, use_limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
