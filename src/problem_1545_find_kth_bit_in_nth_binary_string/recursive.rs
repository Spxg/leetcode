pub struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        fn helper(n: i32, k: i32) -> bool {
            if n == 1 {
                return false;
            }
            let prev = 2i32.pow(n as u32 - 1) - 1;
            match (prev + 1).cmp(&k) {
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Less => {
                    let k = prev - (k - (prev + 1)) + 1;
                    !helper(n - 1, k)
                }
                std::cmp::Ordering::Greater => helper(n - 1, k),
            }
        }
        (u8::from(helper(n, k)) + b'0') as char
    }
}

impl super::Solution for Solution {
    fn find_kth_bit(n: i32, k: i32) -> char {
        Self::find_kth_bit(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
