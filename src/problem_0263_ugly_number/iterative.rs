pub struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        let factors = [2, 3, 5];
        for factor in factors {
            while n % factor == 0 {
                n /= factor;
            }
        }
        n == 1
    }
}

impl super::Solution for Solution {
    fn is_ugly(num: i32) -> bool {
        Self::is_ugly(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
