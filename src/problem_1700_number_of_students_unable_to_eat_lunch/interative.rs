pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut zero = 0;
        let mut one = 0;
        for student in students {
            if student == 1 {
                one += 1;
            } else {
                zero += 1;
            }
        }
        for sandwich in sandwiches {
            if sandwich == 1 {
                if one == 0 {
                    return zero;
                }
                one -= 1;
            } else {
                if zero == 0 {
                    return one;
                }
                zero -= 1;
            }
        }
        0
    }
}

impl super::Solution for Solution {
    fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        Self::count_students(students, sandwiches)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
