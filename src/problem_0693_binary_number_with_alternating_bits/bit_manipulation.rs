pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut prev = n & 1;
        while n > 0 {
            if n & 1 != prev {
                return false;
            }
            prev ^= 1;
            n >>= 1;
        }
        true
    }
}

impl super::Solution for Solution {
    fn has_alternating_bits(n: i32) -> bool {
        Self::has_alternating_bits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
