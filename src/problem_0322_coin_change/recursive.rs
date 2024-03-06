pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        fn helper(coins: &[i32], amount: i32, cache: &mut HashMap<i32, i32>) -> i32 {
            if let Some(result) = cache.get(&amount).copied() {
                return result;
            }

            let mut result = -1;
            for &coin in coins {
                let count = match coin.cmp(&amount) {
                    std::cmp::Ordering::Less => {
                        let count = helper(coins, amount - coin, cache);
                        if count == -1 {
                            continue;
                        }
                        count + 1
                    }
                    std::cmp::Ordering::Equal => 1,
                    std::cmp::Ordering::Greater => continue,
                };
                result = if result == -1 {
                    count
                } else {
                    result.min(count)
                }
            }

            cache.insert(amount, result);
            result
        }

        if amount == 0 {
            return 0;
        }

        helper(&coins, amount, &mut HashMap::new())
    }
}

impl super::Solution for Solution {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        Self::coin_change(coins, amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
