pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }
        let mut fibs = [0, 1];
        for _ in 0..=n - 2 {
            let f = fibs[0] + fibs[1];
            fibs[0] = fibs[1];
            fibs[1] = f;
        }
        fibs[1]
    }
}

impl super::Solution for Solution {
    fn fib(n: i32) -> i32 {
        Self::fib(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
