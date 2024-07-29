pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        if k as usize == points.len() {
            return points;
        }
        points.select_nth_unstable_by_key(k as _, |point| {
            point[0].abs().pow(2) + point[1].abs().pow(2)
        });
        points.truncate(k as _);
        points
    }
}

impl super::Solution for Solution {
    fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Self::k_closest(points, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
