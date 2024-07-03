pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = Vec::with_capacity(s.len());
        for ch in s.chars() {
            match ch {
                '(' => stack.push(0),
                ')' => {
                    let mut val = 0;
                    while let Some(prev) = stack.pop() {
                        if prev == 0 {
                            stack.push(if val == 0 { 1 } else { 2 * val });
                            break;
                        }
                        val += prev;
                    }
                }
                _ => unreachable!(),
            }
        }
        stack.into_iter().sum()
    }
}

impl super::Solution for Solution {
    fn score_of_parentheses(s: String) -> i32 {
        Self::score_of_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
