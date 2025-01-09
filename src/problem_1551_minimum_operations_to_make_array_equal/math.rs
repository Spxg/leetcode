pub struct Solution;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        (n + 1) / 2 * (n - 1 + i32::from(n % 2 == 0)) / 2
    }
}

impl super::Solution for Solution {
    fn min_operations(n: i32) -> i32 {
        Self::min_operations(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
