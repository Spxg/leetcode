pub struct Solution;

impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let count = piles.len() / 3;
        let mut piles = piles;
        piles.select_nth_unstable(count);
        piles[count..].sort_unstable();
        piles[count..].iter().step_by(2).sum()
    }
}

impl super::Solution for Solution {
    fn max_coins(piles: Vec<i32>) -> i32 {
        Self::max_coins(piles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
