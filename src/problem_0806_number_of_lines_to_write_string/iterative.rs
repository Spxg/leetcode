pub struct Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut result = vec![1, 0];
        for byte in s.bytes() {
            let idx = (byte - b'a') as usize;
            if result[1] + widths[idx] <= 100 {
                result[1] += widths[idx];
            } else {
                result[1] = widths[idx];
                result[0] += 1;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        Self::number_of_lines(widths, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
