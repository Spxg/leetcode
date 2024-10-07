pub struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut count = [0; 26];
        for idx in text.bytes() {
            count[(idx - b'a') as usize] += 1;
        }
        count[1]
            .min(count[0])
            .min(count[11] / 2)
            .min(count[14] / 2)
            .min(count[13])
    }
}

impl super::Solution for Solution {
    fn max_number_of_balloons(text: String) -> i32 {
        Self::max_number_of_balloons(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
