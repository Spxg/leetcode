pub struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_lowercase()
    }
}

impl super::Solution for Solution {
    fn to_lower_case(str: String) -> String {
        Self::to_lower_case(str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
