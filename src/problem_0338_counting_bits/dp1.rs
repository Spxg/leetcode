pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let len = n as usize + 1;
        let mut result = Vec::with_capacity(len);
        result.push(0);
        loop {
            if result.len() == len {
                return result;
            }

            for idx in 0..result.len() {
                result.push(result[idx] + 1);
                if result.len() == len {
                    break;
                }
            }
        }
    }
}

impl super::Solution for Solution {
    fn count_bits(num: i32) -> Vec<i32> {
        Self::count_bits(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
