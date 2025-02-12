pub struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut nums = [0; 10];
        for ch in s.chars() {
            if ch.is_numeric() {
                let idx = ch as u8 - b'0';
                nums[idx as usize] = 1;
            }
        }
        nums.iter()
            .enumerate()
            .rev()
            .filter(|&(_, &val)| val == 1)
            .nth(1)
            .map_or(-1, |(x, _)| x as i32)
    }
}

impl super::Solution for Solution {
    fn second_highest(s: String) -> i32 {
        Self::second_highest(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
