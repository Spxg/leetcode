pub struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour = f64::from(hour);
        let minutes = f64::from(minutes);
        let minute_ratio = minutes / 60.;
        let hour_ratio = (hour % 12.) / 12.;

        let angle = minute_ratio
            .mul_add(360., -hour_ratio.mul_add(360., 30. * minute_ratio))
            .abs();

        angle.min(360.0 - angle)
    }
}

impl super::Solution for Solution {
    fn angle_clock(hour: i32, minutes: i32) -> f64 {
        Self::angle_clock(hour, minutes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
