pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;

        for index1 in (0..triangle.len() - 1).rev() {
            for index2 in 0..triangle[index1].len() {
                triangle[index1][index2] +=
                    triangle[index1 + 1][index2].min(triangle[index1 + 1][index2 + 1]);
            }
        }

        triangle[0][0]
    }
}

impl super::Solution for Solution {
    fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        Self::minimum_total(triangle)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
