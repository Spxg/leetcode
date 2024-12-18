pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        fn helper(ch: u8) -> bool {
            matches!(ch, b'a' | b'e' | b'i' | b'o' | b'u')
        }

        let bytes = s.into_bytes();
        let mut sum = bytes[0..k as usize].iter().filter(|&&x| helper(x)).count() as i32;
        let mut result = sum;

        for (left, right) in (0usize..).zip(k as usize..bytes.len()) {
            if result == k {
                return result;
            }
            sum = sum - i32::from(helper(bytes[left])) + i32::from(helper(bytes[right]));
            result = result.max(sum);
        }

        result
    }
}

impl super::Solution for Solution {
    fn max_vowels(s: String, k: i32) -> i32 {
        Self::max_vowels(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
