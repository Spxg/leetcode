pub struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(encoded.len() + 1);
        result.push(first);

        let mut num = first;
        for encoded_num in encoded {
            num ^= encoded_num;
            result.push(num);
        }

        result
    }
}

impl super::Solution for Solution {
    fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        Self::decode(encoded, first)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
