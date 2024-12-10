pub struct Solution;

impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut coins = coins;
        coins.sort_unstable();
        for coin in coins {
            if result >= coin {
                result += coin;
            } else {
                break;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        Self::get_maximum_consecutive(coins)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
