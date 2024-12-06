pub struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut result = String::new();
        let mut chs = [(a, 'a'), (b, 'b'), (c, 'c')];

        loop {
            chs.sort_unstable_by_key(|x| -x.0);

            if let Some((count, ch)) = chs.iter_mut().find(|x| {
                let mut iter = result.chars();
                x.0 != 0 && (iter.next_back() != Some(x.1) || iter.next_back() != Some(x.1))
            }) {
                result.push(*ch);
                *count -= 1;
            } else {
                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        Self::longest_diverse_string(a, b, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
