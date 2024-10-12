pub struct Solution;

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let mut iters = vec![];
        let mut max_size = 0;
        for s in s.split(' ').filter(|x| !x.is_empty()) {
            max_size = max_size.max(s.len());
            iters.push(s.chars());
        }

        let mut result = Vec::with_capacity(max_size);
        for _ in 0..max_size {
            let s = iters
                .iter_mut()
                .map(|iter| iter.next().unwrap_or(' '))
                .collect::<String>();
            result.push(s.trim_end().into());
        }

        result
    }
}

impl super::Solution for Solution {
    fn print_vertically(s: String) -> Vec<String> {
        Self::print_vertically(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
