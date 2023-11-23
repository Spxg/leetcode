pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        fn helper(start: i32, end: i32, cache: &mut HashMap<i32, i32>) -> i32 {
            if start == (start + end) / 2 {
                return 1;
            };
            if let Some(cache) = cache.get(&(end - start)) {
                return *cache;
            }

            let mut result = 0;
            for val in start..(start + end) / 2 {
                let left_tree = helper(start, val, cache);
                let right_tree = helper(val + 1, end, cache);
                result += 2 * left_tree * right_tree;
            }
            if (start + end) % 2 != 0 {
                result += helper(start, (start + end) / 2, cache)
                    * helper((start + end) / 2 + 1, end, cache);
            }
            cache.insert(end - start, result);

            result
        }

        helper(1, n + 1, &mut HashMap::new())
    }
}

impl super::Solution for Solution {
    fn num_trees(n: i32) -> i32 {
        Self::num_trees(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
