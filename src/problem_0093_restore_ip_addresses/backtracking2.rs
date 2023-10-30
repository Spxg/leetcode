pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn helper(s: &[u8], element: &mut Vec<String>, result: &mut Vec<String>) {
            if s.is_empty() && element.len() == 4 {
                result.push(element.join("."));
            }

            for idx in 1..=3.min(s.len()) {
                let slice = &s[0..idx];
                let num = slice.iter().fold(0, |acc, x| acc * 10 + i32::from(*x));
                if slice.len() == 1 || slice.len() > 1 && slice[0] != 0 && num <= 255 {
                    element.push(num.to_string());
                    helper(&s[idx..], element, result);
                    element.pop();
                }
            }
        }

        let s = s.as_bytes().iter().map(|x| *x - b'0').collect::<Vec<_>>();
        let mut result = vec![];
        helper(&s, &mut Vec::new(), &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn restore_ip_addresses(s: String) -> Vec<String> {
        Self::restore_ip_addresses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
