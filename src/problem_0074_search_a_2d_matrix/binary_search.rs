pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut left = 0;
        let mut right = m * n - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            match matrix[mid / n][mid % n].cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                }
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        Self::search_matrix(matrix, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
