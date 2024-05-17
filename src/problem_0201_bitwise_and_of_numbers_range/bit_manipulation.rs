pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut left = left;
        let mut right = right;
        let mut shift = 0;

        while left != right {
            left >>= 1;
            right >>= 1;
            shift += 1;
        }

        left << shift
    }
}

impl super::Solution for Solution {
    fn range_bitwise_and(m: i32, n: i32) -> i32 {
        Self::range_bitwise_and(m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
