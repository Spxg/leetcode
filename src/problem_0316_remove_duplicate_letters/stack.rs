pub struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack = vec![];
        let mut used = [false; 26];
        let mut unused = [0; 26];
        for byte in s.bytes() {
            unused[(byte - b'a') as usize] += 1;
        }
        for byte in s.bytes() {
            let idx1 = (byte - b'a') as usize;
            unused[idx1] -= 1;
            if used[idx1] {
                continue;
            }

            while let Some(&val) = stack.last() {
                let idx2 = (val - b'a') as usize;
                if byte < val && unused[idx2] > 0 {
                    used[idx2] = false;
                    stack.pop();
                } else {
                    break;
                }
            }

            used[idx1] = true;
            stack.push(byte);
        }
        String::from_utf8(stack).unwrap()
    }
}

impl super::Solution for Solution {
    fn remove_duplicate_letters(s: String) -> String {
        Self::remove_duplicate_letters(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
