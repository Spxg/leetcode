pub struct Solution;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max = i32::MIN;

        for (size, num) in (1..).zip(flips) {
            max = num.max(max);
            result += i32::from(max == size);
        }

        result
    }
}

impl super::Solution for Solution {
    fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        Self::num_times_all_blue(flips)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
