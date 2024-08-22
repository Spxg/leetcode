pub struct Solution;

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut group = 0;
        let rest = match n % 4 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 6,
            _ => unreachable!(),
        };

        for num in (1..=n).rev().step_by(4).take((n / 4) as usize) {
            let val = num * (num - 1) / (num - 2) + if group == 0 { 1 } else { -1 } * (num - 3);
            if group == 0 {
                group = val;
            } else {
                group -= val;
            }
        }

        if group == 0 {
            rest
        } else {
            group - rest
        }
    }
}

impl super::Solution for Solution {
    fn clumsy(n: i32) -> i32 {
        Self::clumsy(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
