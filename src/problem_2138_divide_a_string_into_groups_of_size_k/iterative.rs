pub struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut result = vec![];
        let extra = (k - (s.len() as i32 % k)) as usize;
        let s = s.into_bytes();
        for bytes in s.chunks(k as usize) {
            let mut bytes = bytes.to_vec();
            if bytes.len() != k as usize {
                bytes.extend(std::iter::repeat_n(fill as u8, extra));
            }
            result.push(String::from_utf8(bytes).unwrap());
        }
        result
    }
}

impl super::Solution for Solution {
    fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        Self::divide_string(s, k, fill)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
