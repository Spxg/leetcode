pub struct Solution;

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n = n;
        let mut mul = 1;
        let mut result = 0;
        let mut prev = 9;
        while n > 0 {
            let digit = n % 10;
            (result, prev) = if digit <= prev {
                (digit * mul + result, digit)
            } else {
                (digit * mul - 1, digit - 1)
            };
            mul *= 10;
            n /= 10;
        }
        result
    }
}

impl super::Solution for Solution {
    fn monotone_increasing_digits(n: i32) -> i32 {
        Self::monotone_increasing_digits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
