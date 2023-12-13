pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = vec![0; prices.len()];
        let mut min_price = prices[0];
        let len = prices.len();

        for idx in 1..len {
            min_price = min_price.min(prices[idx]);
            result[idx] = (prices[idx] - min_price)
                .max(result[idx - 1] + prices[idx] - prices[idx - 1])
                .max(result[idx - 1]);
        }

        result[len - 1]
    }
}

impl super::Solution for Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        Self::max_profit(prices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
