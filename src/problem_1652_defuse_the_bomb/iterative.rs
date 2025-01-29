pub struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(code.len());
        let length = code.len() as i32;
        for idx in 0..length {
            result.push(match k.cmp(&0) {
                std::cmp::Ordering::Less => {
                    fn helper(idx: i32, val: i32, length: i32) -> usize {
                        ((idx - val + length) % length) as usize
                    }
                    (1..=k.abs()).fold(0, |acc, x| acc + code[helper(idx, x, length)])
                }
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => {
                    fn helper(idx: i32, val: i32, length: i32) -> usize {
                        ((idx + val) % length) as usize
                    }
                    (1..=k).fold(0, |acc, x| acc + code[helper(idx, x, length)])
                }
            });
        }
        result
    }
}

impl super::Solution for Solution {
    fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        Self::decrypt(code, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
