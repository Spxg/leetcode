pub struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = String::with_capacity(command.len());
        let mut al = false;
        for ch in command.chars() {
            match ch {
                'G' => result.push('G'),
                '(' => al = false,
                'a' | 'l' => al = true,
                ')' => {
                    if al {
                        result.push_str("al");
                    } else {
                        result.push('o');
                    }
                }
                _ => unreachable!(),
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn interpret(command: String) -> String {
        Self::interpret(command)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
