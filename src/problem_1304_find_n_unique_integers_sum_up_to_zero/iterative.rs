pub struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        (-(n / 2)..0)
            .chain(i32::from(n % 2 == 0)..=(n / 2))
            .collect()
    }
}

impl super::Solution for Solution {
    fn sum_zero(n: i32) -> Vec<i32> {
        Self::sum_zero(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
