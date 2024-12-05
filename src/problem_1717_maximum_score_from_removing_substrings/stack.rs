pub struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut result = 0;
        let mut stack1 = Vec::with_capacity(s.len());

        for ch in s.chars() {
            if let Some(prev) = stack1.last().copied() {
                if x > y && (prev, ch) == ('a', 'b') {
                    result += x;
                    stack1.pop();
                } else if x <= y && (prev, ch) == ('b', 'a') {
                    result += y;
                    stack1.pop();
                } else {
                    stack1.push(ch);
                }
            } else {
                stack1.push(ch);
            }
        }

        let mut stack2 = Vec::with_capacity(stack1.len());
        for ch in stack1 {
            if let Some(prev) = stack2.last().copied() {
                if (prev, ch) == ('a', 'b') {
                    result += x;
                    stack2.pop();
                } else if (prev, ch) == ('b', 'a') {
                    result += y;
                    stack2.pop();
                } else {
                    stack2.push(ch);
                }
            } else {
                stack2.push(ch);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        Self::maximum_gain(s, x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
