pub struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![0; s.len()];
        for (indice, byte) in indices.into_iter().zip(s.bytes()) {
            result[indice as usize] = byte;
        }
        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn restore_string(s: String, indices: Vec<i32>) -> String {
        Self::restore_string(s, indices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
