pub struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        !n << n.leading_zeros() >> n.leading_zeros()
    }
}

impl super::Solution for Solution {
    fn bitwise_complement(n: i32) -> i32 {
        Self::bitwise_complement(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
