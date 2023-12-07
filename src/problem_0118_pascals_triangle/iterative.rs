pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;

        let mut result = Vec::with_capacity(num_rows);
        result.push(vec![1]);

        for idx in 1..num_rows {
            let mut element = Vec::with_capacity(idx + 1);
            element.push(1);
            for window in result[idx - 1].windows(2) {
                element.push(window[0] + window[1]);
            }
            element.push(1);
            result.push(element);
        }

        result
    }
}

impl super::Solution for Solution {
    fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        Self::generate(num_rows)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
