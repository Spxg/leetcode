pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        !num & ((1 << (32 - num.leading_zeros())) - 1)
    }
}

impl super::Solution for Solution {
    fn find_complement(num: i32) -> i32 {
        Self::find_complement(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
