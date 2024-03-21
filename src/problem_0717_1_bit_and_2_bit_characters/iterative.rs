pub struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut idx = 0;
        while idx < bits.len() {
            if idx == bits.len() - 1 {
                return true;
            }
            if bits[idx] == 1 {
                idx += 2;
            } else {
                idx += 1;
            }
        }
        false
    }
}

impl super::Solution for Solution {
    fn is_one_bit_character(bits: Vec<i32>) -> bool {
        Self::is_one_bit_character(bits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
