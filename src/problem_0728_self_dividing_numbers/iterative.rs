pub struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity((right - left + 1) as _);
        'l: for num in left..=right {
            let mut n = num;
            while n != 0 {
                let x = n % 10;
                if x == 0 || num % x != 0 {
                    continue 'l;
                }
                n /= 10;
            }
            result.push(num);
        }
        result
    }
}

impl super::Solution for Solution {
    fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        Self::self_dividing_numbers(left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
