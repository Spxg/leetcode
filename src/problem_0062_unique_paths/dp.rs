pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut path = vec![vec![1; n]; m];
        for x in 1..m {
            for y in 1..n {
                path[x][y] = path[x - 1][y] + path[x][y - 1];
            }
        }
        path[m - 1][n - 1]
    }
}

impl super::Solution for Solution {
    fn unique_paths(m: i32, n: i32) -> i32 {
        Self::unique_paths(m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
