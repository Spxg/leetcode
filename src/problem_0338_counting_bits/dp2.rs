pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let len = n as usize + 1;
        let mut result = vec![0; len];
        for i in 1..len {
            result[i] = result[i & (i - 1)] + 1;
        }
        result
    }
}

impl super::Solution for Solution {
    fn count_bits(num: i32) -> Vec<i32> {
        Self::count_bits(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
