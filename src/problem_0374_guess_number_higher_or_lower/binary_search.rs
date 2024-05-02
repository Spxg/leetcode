use super::guess;

pub struct Solution;

impl Solution {
    #[allow(non_snake_case, unsafe_code)]
    pub fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        loop {
            let mid = left + (right - left) / 2;
            // SAFETY:
            // Function requirement.
            match unsafe { guess(mid) } {
                0 => return mid,
                1 => left = mid + 1,
                -1 => right = mid - 1,
                _ => unreachable!(),
            }
        }
    }
}

impl super::Solution for Solution {
    fn guess_number(n: i32) -> i32 {
        Self::guessNumber(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
