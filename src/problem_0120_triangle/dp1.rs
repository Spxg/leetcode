pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;

        for index1 in 1..triangle.len() {
            for index2 in 0..=index1 {
                triangle[index1][index2] += if index2 == 0 {
                    triangle[index1 - 1][index2]
                } else if index2 == index1 {
                    triangle[index1 - 1][index2 - 1]
                } else {
                    triangle[index1 - 1][index2 - 1].min(triangle[index1 - 1][index2])
                }
            }
        }

        triangle.pop().unwrap().into_iter().min().unwrap()
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
