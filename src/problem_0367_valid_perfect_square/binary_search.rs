pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut left = 0;
        let mut right = (num / 2) + 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if let Some(value) = mid.checked_mul(mid) {
                match value.cmp(&num) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Equal => return true,
                    std::cmp::Ordering::Greater => {
                        if mid == 0 {
                            break;
                        }
                        right = mid - 1;
                    }
                }
            } else {
                right = mid - 1;
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn is_perfect_square(num: i32) -> bool {
        Self::is_perfect_square(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
