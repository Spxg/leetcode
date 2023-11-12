pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn help(x: f64, n: i32) -> f64 {
            if n == 0 {
                1.0
            } else {
                let half = help(x, n / 2);
                let result = half * half;
                if n % 2 == 0 {
                    result
                } else {
                    result * x
                }
            }
        }

        let x = if n > 0 { x } else { 1.0 / x };
        help(x, n.abs())
    }
}

impl super::Solution for Solution {
    fn my_pow(x: f64, n: i32) -> f64 {
        Self::my_pow(x, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
