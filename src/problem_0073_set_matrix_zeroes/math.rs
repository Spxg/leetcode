pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let idxs: Vec<_> = matrix
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, &ele)| ele == 0)
            .map(|(idx, _)| idx)
            .collect();

        for idx in idxs {
            let line = idx / n;
            let col = idx % n;
            (0..m).for_each(|x| matrix[x][col] = 0);
            (0..n).for_each(|x| matrix[line][x] = 0);
        }
    }
}

impl super::Solution for Solution {
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        Self::set_zeroes(matrix);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
