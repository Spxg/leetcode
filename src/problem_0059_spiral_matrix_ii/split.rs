pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut count = 1;
        let mut matrix = vec![vec![0; n]; n];
        for idx in 0..=(n / 2) {
            if let Some((first, ele)) = matrix[idx..n - idx].split_first_mut() {
                for x in &mut first[idx..n - idx] {
                    *x = count;
                    count += 1;
                }
                if let Some((last, mids)) = ele.split_last_mut() {
                    for mid in &mut *mids {
                        if let Some(x) = mid[idx..n - idx].last_mut() {
                            *x = count;
                            count += 1;
                        }
                    }
                    for x in last[idx..n - idx].iter_mut().rev() {
                        *x = count;
                        count += 1;
                    }
                    for mid in mids.iter_mut().rev() {
                        if let Some(x) = mid[idx..n - idx]
                            .split_last_mut()
                            .and_then(|(_, ele)| ele.first_mut())
                        {
                            *x = count;
                            count += 1;
                        }
                    }
                }
            }
        }
        matrix
    }
}

impl super::Solution for Solution {
    fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        Self::generate_matrix(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
