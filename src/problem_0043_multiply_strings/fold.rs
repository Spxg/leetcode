pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1.eq("0") || num2.eq("0") {
            return "0".into();
        }

        let result = (0..)
            .zip(num2.as_bytes().iter().rev().map(|x| x - b'0'))
            .fold(vec![0], |acc, (count, x)| {
                let mut result = vec![];
                let mut acc_rev = acc.into_iter();
                let mut carry = 0;

                for ch in std::iter::repeat_n(0, count)
                    .chain(num1.as_bytes().iter().rev().map(|x| x - b'0'))
                {
                    let caculate = x * ch + acc_rev.next().unwrap_or_default() + carry;
                    let remainder = caculate % 10;
                    carry = caculate / 10;
                    result.push(remainder);
                }

                if carry != 0 {
                    result.push(carry);
                }

                result
            });

        result
            .into_iter()
            .rev()
            .map(|x| char::from_digit(u32::from(x), 10).unwrap())
            .collect()
    }
}

impl super::Solution for Solution {
    fn multiply(num1: String, num2: String) -> String {
        Self::multiply(num1, num2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
