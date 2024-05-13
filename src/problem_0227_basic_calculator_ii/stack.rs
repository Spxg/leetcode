pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut ops = vec![];

        let mut num = 0;
        for ch in s.bytes().chain(std::iter::once(b'+')) {
            match ch {
                b' ' => continue,
                b'+' | b'-' | b'*' | b'/' => {
                    if let Some(&x) = ops.last() {
                        if x == b'*' {
                            num *= stack.pop().unwrap();
                            ops.pop();
                        } else if x == b'/' {
                            num = stack.pop().unwrap() / num;
                            ops.pop();
                        }
                    }
                    stack.push(num);
                    ops.push(ch);
                    num = 0;
                }
                n => num = num * 10 + i32::from(n - b'0'),
            }
        }
        stack.push(num);

        let mut result = stack[0];
        for (num, op) in stack.into_iter().skip(1).zip(ops) {
            result += if op == b'-' { -num } else { num };
        }
        result
    }
}

impl super::Solution for Solution {
    fn calculate(s: String) -> i32 {
        Self::calculate(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
