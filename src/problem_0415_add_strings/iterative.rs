pub struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut result = vec![];
        let mut carry = 0;

        let mut num1 = num1.bytes().rev().map(|x| x - b'0');
        let mut num2 = num2.bytes().rev().map(|x| x - b'0');

        loop {
            let sum = match (num1.next(), num2.next()) {
                (None, None) => {
                    if carry != 0 {
                        result.push(carry + b'0');
                    }
                    break;
                }
                (Some(rest), None) | (None, Some(rest)) => rest,
                (Some(a), Some(b)) => a + b,
            } + carry;
            result.push(sum % 10 + b'0');
            carry = sum / 10;
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn add_strings(num1: String, num2: String) -> String {
        Self::add_strings(num1, num2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
