pub struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut chars = [0; 26];
        s.bytes().for_each(|x| chars[(x - b'a') as usize] += 1);

        let zero = chars[usize::from(b'z' - b'a')];
        let two = chars[usize::from(b'w' - b'a')];
        let four = chars[usize::from(b'u' - b'a')];
        let six = chars[usize::from(b'x' - b'a')];
        let eight = chars[usize::from(b'g' - b'a')];
        let one = chars[usize::from(b'o' - b'a')] - zero - two - four;
        let three = chars[usize::from(b'h' - b'a')] - eight;
        let five = chars[usize::from(b'f' - b'a')] - four;
        let seven = chars[usize::from(b'v' - b'a')] - five;
        let nine = chars[usize::from(b'i' - b'a')] - six - eight - five;

        let digit_counts = [zero, one, two, three, four, five, six, seven, eight, nine];
        let mut result = Vec::with_capacity(digit_counts.iter().sum());
        for (digit, count) in (b'0'..).zip(digit_counts) {
            (0..count).for_each(|_| result.push(digit));
        }
        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn original_digits(s: String) -> String {
        Self::original_digits(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
