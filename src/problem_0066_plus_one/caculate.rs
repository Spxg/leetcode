pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut result = vec![];

        for digit in digits.into_iter().rev() {
            let caculate = digit + carry;
            result.push(caculate % 10);
            carry = caculate / 10;
        }

        if carry != 0 {
            result.push(carry);
        }

        result.reverse();
        result
    }
}

impl super::Solution for Solution {
    fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        Self::plus_one(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
