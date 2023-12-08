pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut result = vec![1; row_index + 1];
        for idx in 1..=row_index {
            (1..idx).rev().for_each(|x| result[x] += result[x - 1]);
        }
        result
    }
}

impl super::Solution for Solution {
    fn get_row(row_index: i32) -> Vec<i32> {
        Self::get_row(row_index)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
