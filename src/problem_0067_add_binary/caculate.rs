pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = vec![];
        let mut iter1 = a.as_bytes().iter().rev().map(|&x| x - b'0');
        let mut iter2 = b.as_bytes().iter().rev().map(|&x| x - b'0');

        let mut carry = 0;
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(lhs), Some(rhs)) => {
                    let calculate = lhs + rhs + carry;
                    carry = calculate / 2;
                    result.push(calculate % 2);
                }
                (Some(remainder), None) | (None, Some(remainder)) => {
                    let calculate = remainder + carry;
                    carry = calculate / 2;
                    result.push(calculate % 2);
                }
                (None, None) => {
                    if carry != 0 {
                        result.push(carry);
                    }
                    break result
                        .into_iter()
                        .rev()
                        .map(|x| char::from_digit(u32::from(x), 2).unwrap())
                        .collect();
                }
            }
        }
    }
}

impl super::Solution for Solution {
    fn add_binary(a: String, b: String) -> String {
        Self::add_binary(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
