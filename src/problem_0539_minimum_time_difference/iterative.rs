pub struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = time_points
            .into_iter()
            .map(|x| {
                let (h, m) = x.split_once(':').unwrap();
                h.parse::<i32>().unwrap() * 60 + m.parse::<i32>().unwrap()
            })
            .collect::<Vec<i32>>();
        minutes.sort_unstable();

        let mut result = minutes[0] - minutes[minutes.len() - 1] + 24 * 60;
        for windows in minutes.windows(2) {
            let &[a, b] = windows else { unreachable!() };
            result = result.min(b - a);
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_min_difference(time_points: Vec<String>) -> i32 {
        Self::find_min_difference(time_points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
