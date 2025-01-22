pub struct Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let (hour, minutes) = time.split_once(':').unwrap();
        let mut chars = hour.chars();
        let hour_1 = chars
            .next()
            .filter(|x| x.is_numeric())
            .map(|x| x as u8 - b'0');
        let hour_2 = chars
            .next()
            .filter(|x| x.is_numeric())
            .map(|x| x as u8 - b'0');
        let mut chars = minutes.chars();
        let minute_1 = chars
            .next()
            .filter(|x| x.is_numeric())
            .map(|x| x as u8 - b'0');
        let minute_2 = chars
            .next()
            .filter(|x| x.is_numeric())
            .map(|x| x as u8 - b'0');

        let hour = match (hour_1, hour_2) {
            (None, None) => 23,
            (None, Some(rhs)) => {
                if rhs <= 3 {
                    20 + rhs
                } else {
                    10 + rhs
                }
            }
            (Some(lhs), None) => {
                if lhs == 2 {
                    23
                } else {
                    lhs * 10 + 9
                }
            }
            (Some(lhs), Some(rhs)) => lhs * 10 + rhs,
        };

        let minutes = match (minute_1, minute_2) {
            (None, None) => 59,
            (None, Some(rhs)) => 50 + rhs,
            (Some(lhs), None) => lhs * 10 + 9,
            (Some(lhs), Some(rhs)) => lhs * 10 + rhs,
        };

        format!("{hour:02}:{minutes:02}")
    }
}

impl super::Solution for Solution {
    fn maximum_time(time: String) -> String {
        Self::maximum_time(time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
