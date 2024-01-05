pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        let mut result = 0;

        for _ in 0..32 {
            result = (result << 1) | x & 1;
            x >>= 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn reverse_bits(n: u32) -> u32 {
        Self::reverse_bits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
