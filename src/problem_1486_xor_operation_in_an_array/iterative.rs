pub struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n).fold(0, |acc, x| acc ^ (start + 2 * x))
    }
}

impl super::Solution for Solution {
    fn xor_operation(n: i32, start: i32) -> i32 {
        Self::xor_operation(n, start)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
