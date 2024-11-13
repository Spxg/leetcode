pub struct Solution;

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut result = 0;
        let mut flip = '0';

        for num in target.chars() {
            result += i32::from(num != flip);
            flip = num;
        }

        result
    }
}

impl super::Solution for Solution {
    fn min_flips(target: String) -> i32 {
        Self::min_flips(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
