pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result: i32 = 0;
        for &value in column_title.as_bytes() {
            result = result * 26 + i32::from(value - b'A' + 1);
        }
        result
    }
}

impl super::Solution for Solution {
    fn title_to_number(s: String) -> i32 {
        Self::title_to_number(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
