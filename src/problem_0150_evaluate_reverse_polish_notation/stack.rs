pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            let token = token.as_str();
            let result = match token {
                "+" | "-" | "*" | "/" => {
                    let (rhs, lhs) = (stack.pop().unwrap(), stack.pop().unwrap());
                    match token {
                        "+" => lhs + rhs,
                        "-" => lhs - rhs,
                        "*" => lhs * rhs,
                        "/" => lhs / rhs,
                        _ => unreachable!(),
                    }
                }
                number => number.parse::<i32>().unwrap(),
            };
            stack.push(result);
        }
        stack.pop().unwrap()
    }
}

impl super::Solution for Solution {
    fn eval_rpn(tokens: Vec<String>) -> i32 {
        Self::eval_rpn(tokens)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
