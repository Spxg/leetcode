pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();

        for x in 0..m {
            for y in 0..n {
                let from_left = (x > 0).then(|| grid[x - 1][y]);
                let from_up = (y > 0).then(|| grid[x][y - 1]);
                match (from_left, from_up) {
                    (None, None) => continue,
                    (Some(val), None) | (None, Some(val)) => grid[x][y] += val,
                    (Some(val1), Some(val2)) => grid[x][y] += val1.min(val2),
                }
            }
        }

        grid[m - 1][n - 1]
    }
}

impl super::Solution for Solution {
    fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        Self::min_path_sum(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
