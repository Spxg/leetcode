pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }

        let mut left = 0;
        let mut right = (x / 2) + 1;
        let mut result = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            match mid.cmp(&(x / mid)) {
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                    result = mid;
                }
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Greater => {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn my_sqrt(x: i32) -> i32 {
        Self::my_sqrt(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
