pub struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // same as `n > 0 && n.count_ones() == 1 && n.trailing_zeros() % 2 == 0`
        n > 0 && n & (n - 1) == 0 && (n - 1) % 3 == 0
    }
}

impl super::Solution for Solution {
    fn is_power_of_four(num: i32) -> bool {
        Self::is_power_of_four(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
