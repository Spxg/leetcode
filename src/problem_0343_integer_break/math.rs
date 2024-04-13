pub struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }

        let (mut x, mut y) = if n % 2 == 0 {
            (n / 2, 0)
        } else {
            (n / 2 - 1, 1)
        };

        y += 2 * (x / 3);
        x %= 3;

        2i32.pow(x as u32) * 3i32.pow(y as u32)
    }
}

impl super::Solution for Solution {
    fn integer_break(n: i32) -> i32 {
        Self::integer_break(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
