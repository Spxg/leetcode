pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut parts = vec![vec![]; num_rows];
        for chars in s.as_bytes().chunks(num_rows + num_rows - 2) {
            let mut chunk = chars.chunks(num_rows);
            if let Some(chunk1) = chunk.next() {
                (0..num_rows)
                    .zip(chunk1)
                    .for_each(|(idx, char)| parts[idx].push(*char));
            }
            if let Some(chunk2) = chunk.next() {
                (1..num_rows - 1)
                    .rev()
                    .zip(chunk2)
                    .for_each(|(idx, char)| parts[idx].push(*char));
            }
        }
        String::from_utf8(parts.into_iter().flatten().collect()).unwrap()
    }
}

impl super::Solution for Solution {
    fn convert(s: String, num_rows: i32) -> String {
        Self::convert(s, num_rows)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
