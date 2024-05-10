pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len() - 1, mat[0].len() - 1);
        let (mut x, mut y) = (0, 0);
        let mut result = Vec::with_capacity(mat.len() * mat[0].len());
        let mut up = true;

        loop {
            result.push(mat[x][y]);
            if x == m && y == n {
                break;
            }
            if x == 0 && up {
                match y.cmp(&n) {
                    std::cmp::Ordering::Less => y += 1,
                    std::cmp::Ordering::Equal => x += 1,
                    std::cmp::Ordering::Greater => unreachable!(),
                }
                up = false;
            } else if x == m && !up {
                y += 1;
                up = true;
            } else if y == 0 && !up {
                match x.cmp(&m) {
                    std::cmp::Ordering::Less => x += 1,
                    std::cmp::Ordering::Equal => y += 1,
                    std::cmp::Ordering::Greater => unreachable!(),
                }
                up = true;
            } else if y == n && up {
                x += 1;
                up = false;
            } else if up {
                x -= 1;
                y += 1;
            } else {
                x += 1;
                y -= 1;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_diagonal_order(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
