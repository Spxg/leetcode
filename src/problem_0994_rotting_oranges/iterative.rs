pub struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut fresh = 0;
        let mut rotten = vec![];
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());

        for (x, row) in grid.iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                match val {
                    0 => continue,
                    1 => fresh += 1,
                    2 => rotten.push((x, y)),
                    _ => unreachable!(),
                }
            }
        }

        while fresh != 0 {
            if rotten.is_empty() {
                return -1;
            }

            for (x, y) in std::mem::take(&mut rotten) {
                if x > 0 && grid[x - 1][y] == 1 {
                    fresh -= 1;
                    grid[x - 1][y] = 2;
                    rotten.push((x - 1, y));
                }
                if x < m - 1 && grid[x + 1][y] == 1 {
                    fresh -= 1;
                    grid[x + 1][y] = 2;
                    rotten.push((x + 1, y));
                }
                if y > 0 && grid[x][y - 1] == 1 {
                    fresh -= 1;
                    grid[x][y - 1] = 2;
                    rotten.push((x, y - 1));
                }
                if y < n - 1 && grid[x][y + 1] == 1 {
                    fresh -= 1;
                    grid[x][y + 1] = 2;
                    rotten.push((x, y + 1));
                }
            }

            result += 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        Self::oranges_rotting(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
