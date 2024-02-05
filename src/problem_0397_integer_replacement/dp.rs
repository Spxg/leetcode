pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        fn helper(n: u32, map: &mut HashMap<u32, i32>) -> i32 {
            if let Some(&step) = map.get(&n) {
                return step;
            }

            let step = if n % 2 == 0 {
                helper(n / 2, map) + 1
            } else {
                helper(n + 1, map).min(helper(n - 1, map)) + 1
            };

            map.insert(n, step);

            step
        }
        helper(n as _, &mut HashMap::from([(1, 0)]))
    }
}

impl super::Solution for Solution {
    fn integer_replacement(n: i32) -> i32 {
        Self::integer_replacement(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
