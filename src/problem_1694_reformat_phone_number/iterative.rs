pub struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut result = vec![];
        let number = number
            .chars()
            .filter(|&x| x != ' ' && x != '-')
            .collect::<String>();
        let mut idx = 0;
        let mut len = number.len();

        loop {
            if len <= 4 {
                if len < 4 {
                    result.push(&number[idx..idx + len]);
                } else {
                    result.push(&number[idx..idx + 2]);
                    result.push(&number[idx + 2..idx + 4]);
                }
                break;
            }
            result.push(&number[idx..idx + 3]);
            idx += 3;
            len -= 3;
        }

        result.join("-")
    }
}

impl super::Solution for Solution {
    fn reformat_number(number: String) -> String {
        Self::reformat_number(number)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
