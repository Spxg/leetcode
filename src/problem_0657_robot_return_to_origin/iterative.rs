pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut pos = (0, 0);
        for ch in moves.chars() {
            match ch {
                'R' => pos.0 += 1,
                'L' => pos.0 -= 1,
                'U' => pos.1 += 1,
                'D' => pos.1 -= 1,
                _ => unreachable!(),
            }
        }
        pos == (0, 0)
    }
}

impl super::Solution for Solution {
    fn judge_circle(moves: String) -> bool {
        Self::judge_circle(moves)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
