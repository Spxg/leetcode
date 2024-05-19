pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // 3^19 < i32::MAX
        // 3^20 > i32::MAX
        n > 0 && (1_162_261_467 % n == 0)
    }
}

impl super::Solution for Solution {
    fn is_power_of_three(n: i32) -> bool {
        Self::is_power_of_three(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
