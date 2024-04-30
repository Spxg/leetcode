pub struct Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }

        let mut result = 1;
        for div in 2..=f64::from(num).sqrt() as i32 {
            if num % div == 0 {
                result += div + num / div;
            }
        }
        result == num
    }
}

impl super::Solution for Solution {
    fn check_perfect_number(num: i32) -> bool {
        Self::check_perfect_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
