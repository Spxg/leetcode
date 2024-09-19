pub struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut result = Vec::with_capacity(s.len());
        let mut stack = Vec::new();
        let mut idx = 0;

        for ch in s.chars() {
            match ch {
                '(' => stack.push(idx),
                ')' => {
                    if let Some(pos) = stack.pop() {
                        result[pos..].reverse();
                    }
                }
                _ => {
                    result.push(ch as u8);
                    idx += 1;
                }
            }
        }

        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn reverse_parentheses(s: String) -> String {
        Self::reverse_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
