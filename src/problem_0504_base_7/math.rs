pub struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let neg = num < 0;
        let mut num = num.abs();
        let mut result = vec![];
        loop {
            result.push((num % 7) as u8 + b'0');
            num /= 7;
            if num == 0 {
                break;
            }
        }
        result
            .into_iter()
            .chain(neg.then_some(b'-'))
            .rev()
            .map(char::from)
            .collect()
    }
}

impl super::Solution for Solution {
    fn convert_to_base7(num: i32) -> String {
        Self::convert_to_base7(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
