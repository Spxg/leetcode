pub struct Solution;

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let (left, right) = equation.split_once('=').unwrap();

        let mut lhs = 0;
        let mut rhs = 0;

        for (rev, s) in [(false, left), (true, right)] {
            let mut positive = true;
            let mut start = 0;
            let mut iter = s.bytes().chain([b'+']).enumerate();
            while let Some((idx, val)) = iter.find(|x| x.1 == b'+' || x.1 == b'-') {
                let sign = if positive { 1 } else { -1 };
                let sign = if rev { -sign } else { sign };
                positive = val == b'+';

                let op = &s[start..idx];
                if let Some(x) = op.strip_suffix('x') {
                    lhs += x.parse::<i32>().unwrap_or(1) * sign;
                } else {
                    rhs -= op.parse::<i32>().unwrap_or(0) * sign;
                }

                start = idx + 1;
            }
        }

        if lhs == 0 && rhs == 0 {
            "Infinite solutions".into()
        } else if lhs == 0 || rhs % lhs != 0 {
            "No solution".into()
        } else {
            format!("x={}", rhs / lhs)
        }
    }
}

impl super::Solution for Solution {
    fn solve_equation(equation: String) -> String {
        Self::solve_equation(equation)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
