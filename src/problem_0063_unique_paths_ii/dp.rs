pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut path = obstacle_grid;
        let m = path.len();
        let n = path[0].len();

        for x in 0..m {
            for y in 0..n {
                path[x][y] = if path[x][y] == 1 {
                    0
                } else {
                    let from_left = (x > 0).then(|| path[x - 1][y]);
                    let from_up = (y > 0).then(|| path[x][y - 1]);
                    match (from_left, from_up) {
                        (None, None) => 1,
                        (Some(val), None) | (None, Some(val)) => val,
                        (Some(val1), Some(val2)) => val1 + val2,
                    }
                }
            }
        }
        path[m - 1][n - 1]
    }
}

impl super::Solution for Solution {
    fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        Self::unique_paths_with_obstacles(obstacle_grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
