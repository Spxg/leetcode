pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut result = vec![0, 1];
        for offset in 1..n {
            result = result
                .iter()
                .copied()
                .chain(result.iter().rev().copied().map(|x| x | 1 << offset))
                .collect();
        }
        result
    }
}

impl super::Solution for Solution {
    fn gray_code(n: i32) -> Vec<i32> {
        Self::gray_code(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
