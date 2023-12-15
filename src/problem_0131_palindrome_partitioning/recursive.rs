pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn helpr(s: &[u8], result: &mut Vec<Vec<String>>, element: &mut Vec<String>) {
            if s.is_empty() {
                result.push(element.clone());
            }

            for idx in 1..=s.len() {
                let substring = String::from_utf8(s[0..idx].to_vec()).unwrap();
                let bytes = substring.as_bytes();
                if bytes.iter().zip(bytes.iter().rev()).all(|(a, b)| a == b) {
                    element.push(substring);
                    helpr(&s[idx..], result, element);
                    element.pop();
                }
            }
        }

        let mut result = vec![];
        helpr(s.as_bytes(), &mut result, &mut Vec::new());
        result
    }
}

impl super::Solution for Solution {
    fn partition(s: String) -> Vec<Vec<String>> {
        Self::partition(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
