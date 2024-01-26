pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        chars.push('0');
        let mut pointer = 0;
        let mut count = 1;

        for i in 0..chars.len() - 1 {
            if chars[i] == chars[i + 1] {
                count += 1;
            } else {
                if count != 1 {
                    for ch in count.to_string().chars() {
                        pointer += 1;
                        chars[pointer] = ch;
                    }
                }
                pointer += 1;
                chars[pointer] = chars[i + 1];
                count = 1;
            }
        }

        pointer as _
    }
}

impl super::Solution for Solution {
    fn compress(chars: &mut Vec<char>) -> i32 {
        Self::compress(chars)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
