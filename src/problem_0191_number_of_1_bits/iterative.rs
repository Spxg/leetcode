pub struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        let mut result = 0;
        for offset in 0..32 {
            result += (n & (1 << offset)) >> offset;
        }
        result as i32
    }
}

impl super::Solution for Solution {
    fn hamming_weight(n: u32) -> i32 {
        Self::hammingWeight(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
