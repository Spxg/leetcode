pub struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut idx = 0;
        let mut count = 0;
        while idx < data.len() && count >= 0 {
            let bits = (data[idx] & 0b1111_1000) >> 3;
            let ones = [0b10000, 0b01000, 0b00100, 0b00010, 0b00001]
                .into_iter()
                .map(|mask| bits & mask)
                .take_while(|&x| x != 0)
                .count() as i32;
            match ones {
                0 if count == 0 => (),
                1 => count -= 1,
                2..=4 if count == 0 => count = ones - 1,
                _ => return false,
            }
            idx += 1;
        }
        count == 0
    }
}

impl super::Solution for Solution {
    fn valid_utf8(data: Vec<i32>) -> bool {
        Self::valid_utf8(data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
