pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idx1 = 0;
        let mut idx2 = numbers.len() - 1;
        loop {
            match numbers[idx2].cmp(&(target - numbers[idx1])) {
                std::cmp::Ordering::Less => idx1 += 1,
                std::cmp::Ordering::Equal => return vec![idx1 as i32 + 1, idx2 as i32 + 1],
                std::cmp::Ordering::Greater => idx2 -= 1,
            }
        }
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
