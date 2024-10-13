pub struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut product = 1;
        let mut sum = 0;
        let mut n = n;
        while n != 0 {
            sum += n % 10;
            product *= n % 10;
            n /= 10;
        }
        product - sum
    }
}

impl super::Solution for Solution {
    fn subtract_product_and_sum(n: i32) -> i32 {
        Self::subtract_product_and_sum(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
