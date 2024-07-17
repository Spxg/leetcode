pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        for ch in s.chars() {
            if let Some(last) = stack.last().copied() {
                if last == '(' && ch == ')' {
                    stack.pop();
                } else {
                    stack.push(ch);
                }
            } else {
                stack.push(ch);
            }
        }

        stack.len() as _
    }
}

impl super::Solution for Solution {
    fn min_add_to_make_valid(s: String) -> i32 {
        Self::min_add_to_make_valid(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
