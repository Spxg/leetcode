pub struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut stack1 = vec![];
        let mut stack2 = vec![];
        for ch1 in s.chars() {
            if ch1 == '#' {
                stack1.pop();
            } else {
                stack1.push(ch1);
            }
        }
        for ch2 in t.chars() {
            if ch2 == '#' {
                stack2.pop();
            } else {
                stack2.push(ch2);
            }
        }
        stack1 == stack2
    }
}

impl super::Solution for Solution {
    fn backspace_compare(s: String, t: String) -> bool {
        Self::backspace_compare(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
