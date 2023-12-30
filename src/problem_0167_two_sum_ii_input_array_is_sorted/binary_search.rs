pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (idx1, &number) in numbers.iter().enumerate() {
            if let Ok(idx2) = numbers[idx1 + 1..].binary_search(&(target - number)) {
                return vec![idx1 as i32 + 1, idx2 as i32 + idx1 as i32 + 2];
            }
        }
        unreachable!()
    }
}

impl super::Solution for Solution {
    fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(numbers, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
