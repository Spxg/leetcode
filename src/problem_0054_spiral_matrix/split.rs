pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut result = vec![];

        for (line, col) in (0..=(m / 2)).zip(0..=(n / 2)) {
            if let Some((first, rest)) = matrix[line..m - line].split_first() {
                first[col..n - col].iter().for_each(|&x| result.push(x));
                if let Some((last, mids)) = rest.split_last() {
                    for mid in mids {
                        if let Some(&x) = mid[col..n - col].last() {
                            result.push(x);
                        }
                    }
                    last[col..n - col]
                        .iter()
                        .rev()
                        .for_each(|&x| result.push(x));
                    for mid in mids.iter().rev() {
                        if let Some(&x) = mid[col..n - col]
                            .split_last()
                            .and_then(|(_, ele)| ele.first())
                        {
                            result.push(x);
                        }
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        Self::spiral_order(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
