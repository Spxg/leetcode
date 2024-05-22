pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        ((-1.0 + 8.0f64.mul_add(f64::from(n), 1.0).sqrt()) / 2.0) as i32
    }
}

impl super::Solution for Solution {
    fn arrange_coins(n: i32) -> i32 {
        Self::arrange_coins(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
