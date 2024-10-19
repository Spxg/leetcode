pub struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let pointer1 = &coordinates[0];
        let pointer2 = &coordinates[1];
        let sub_y = pointer2[1] - pointer1[1];
        let sub_x = pointer2[0] - pointer1[0];

        for pointer in &coordinates[2..] {
            let new_sub_y = pointer[1] - pointer1[1];
            let new_sub_x = pointer[0] - pointer1[0];
            if sub_y * new_sub_x != sub_x * new_sub_y {
                return false;
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        Self::check_straight_line(coordinates)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
