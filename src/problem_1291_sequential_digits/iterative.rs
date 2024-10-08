pub struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        #[rustfmt::skip]
        #[allow(clippy::unreadable_literal)]
        let all: [i32; 45] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            12, 23, 34, 45, 56, 67, 78, 89,
            123, 234, 345, 456, 567, 678, 789,
            1234, 2345, 3456, 4567, 5678, 6789,
            12345, 23456, 34567, 45678, 56789,
            123456, 234567, 345678, 456789,
            1234567, 2345678, 3456789,
            12345678, 23456789,
            123456789
        ];
        all.into_iter().filter(|&x| x >= low && x <= high).collect()
    }
}

impl super::Solution for Solution {
    fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        Self::sequential_digits(low, high)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
