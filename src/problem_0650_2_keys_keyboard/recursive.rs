pub struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        fn helper(n: i32) -> i32 {
            if n == 1 {
                return 0;
            }
            let sqrt = f64::from(n).sqrt() as i32;
            for x in 2..=sqrt {
                if n % x == 0 {
                    return helper(n / x) + x;
                }
            }
            n
        }
        helper(n)
    }
}

impl super::Solution for Solution {
    fn min_steps(n: i32) -> i32 {
        Self::min_steps(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
