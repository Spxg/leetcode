pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let mut result = 0;
        let mut count = 0;

        for num in nums.windows(3) {
            let &[a, b, c] = num else { unreachable!() };
            if b - a == c - b {
                count += 1;
            } else {
                result += (count) * (count + 1) / 2;
                count = 0;
            }
        }

        if count != 0 {
            result += (count) * (count + 1) / 2;
        }

        result
    }
}

impl super::Solution for Solution {
    fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        Self::number_of_arithmetic_slices(a)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
