pub struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        fn helper(num: i32, result: &mut String) {
            if num == 0 {
                return;
            }
            let num = num - 1;
            helper(num / 26, result);
            result.push(((num % 26) as u8 + b'A') as char);
        }
        let mut result = String::new();
        helper(column_number, &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn convert_to_title(n: i32) -> String {
        Self::convert_to_title(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
