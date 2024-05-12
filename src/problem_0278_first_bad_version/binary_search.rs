use super::Solution as _;

pub struct Solution {
    bad: i32,
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = (right - left) / 2 + left;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        right
    }
}

impl super::Solution for Solution {
    fn new(bad: i32) -> Self {
        Self { bad }
    }

    #[allow(non_snake_case)] // Expected.
    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.bad
    }

    fn first_bad_version(&self, n: i32) -> i32 {
        self.first_bad_version(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
