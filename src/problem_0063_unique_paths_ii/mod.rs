pub mod dp;

pub trait Solution {
    fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[0, 0, 0], [0, 1, 0], [0, 0, 0]] as &dyn Matrix<_>, 2)];

        for (obstacle_grid, expected) in test_cases {
            assert_eq!(
                S::unique_paths_with_obstacles(obstacle_grid.to_vec()),
                expected
            );
        }
    }
}
