pub struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1
    }
}

impl super::Solution for Solution {
    fn number_of_matches(n: i32) -> i32 {
        Self::number_of_matches(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
