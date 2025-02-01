pub struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut result = vec![];
        let mut split1 = text.split(' ');
        let mut split2 = text.split(' ').skip(1);
        loop {
            match (split1.next(), split2.next()) {
                (None, None) => break,
                (None, Some(_)) => unreachable!(),
                (Some(_), None) => break,
                (Some(x), Some(y)) => {
                    if x == first && y == second {
                        if let Some(n) = split2.clone().next() {
                            result.push(n.to_string());
                        }
                    }
                }
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        Self::find_ocurrences(text, first, second)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
