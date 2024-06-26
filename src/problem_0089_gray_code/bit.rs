pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..1 << n).map(|x| x ^ (x >> 1)).collect()
    }
}

impl super::Solution for Solution {
    fn gray_code(n: i32) -> Vec<i32> {
        Self::gray_code(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
