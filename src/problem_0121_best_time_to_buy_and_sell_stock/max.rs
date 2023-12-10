pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut buy = prices[0];

        for price in prices {
            buy = buy.min(price);
            result = result.max(price - buy);
        }

        result
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
