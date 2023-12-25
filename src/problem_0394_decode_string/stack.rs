pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<char> = vec![];

        for ch in s.chars() {
            if ch == ']' {
                let pos = stack.iter().rposition(|x| x.eq(&'[')).unwrap();
                let chars = stack.drain(pos..).skip(1).collect::<Vec<_>>();

                let mut repeat = 0;
                let mut mul = 1;
                while let Some(x) = stack.pop() {
                    if !x.is_numeric() {
                        stack.push(x);
                        break;
                    }
                    repeat += usize::from(x as u8 - b'0') * mul;
                    mul *= 10;
                }

                stack.extend(chars.iter().copied().cycle().take(repeat * chars.len()));
            } else {
                stack.push(ch);
            }
        }

        stack.into_iter().collect()
    }
}

impl super::Solution for Solution {
    fn decode_string(s: String) -> String {
        Self::decode_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
