pub struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".into();
        }

        let num = u32::from_le_bytes(num.to_le_bytes());
        let mut result = Vec::with_capacity(8);
        for offset in (0..32).step_by(4).rev() {
            let ch = match (num & (0xf << offset)) >> offset {
                v @ 0..=9 => v as u8 + b'0',
                v => v as u8 - 10 + b'a',
            };
            if !result.is_empty() || ch != b'0' {
                result.push(ch);
            }
        }
        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn to_hex(num: i32) -> String {
        Self::to_hex(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
