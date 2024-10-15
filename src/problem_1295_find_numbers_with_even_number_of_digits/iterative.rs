pub struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|&x| {
                let mut n = x;
                let mut digits = 0;
                while n != 0 {
                    n /= 10;
                    digits += 1;
                }
                digits % 2 == 0
            })
            .count() as _
    }
}

impl super::Solution for Solution {
    fn find_numbers(nums: Vec<i32>) -> i32 {
        Self::find_numbers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
