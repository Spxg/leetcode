pub mod dp;
pub mod fold;

pub trait Solution {
    fn max_profit(prices: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[7, 1, 5, 3, 6, 4] as &[_], 7),
            (&[1, 2, 3, 4, 5], 4),
            (&[7, 6, 4, 3, 1], 0),
        ];

        for (prices, expected) in test_cases {
            assert_eq!(S::max_profit(prices.to_vec()), expected);
        }
    }
}
