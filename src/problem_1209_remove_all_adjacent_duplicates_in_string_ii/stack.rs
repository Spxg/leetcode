pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack = Vec::with_capacity(s.len());
        for ch in s.chars() {
            if let Some((prev, count)) = stack.last_mut() {
                if *prev == ch {
                    *count += 1;
                    if *count == k {
                        stack.pop();
                    }
                    continue;
                }
            }
            stack.push((ch, 1));
        }
        stack
            .into_iter()
            .flat_map(|(ch, count)| std::iter::repeat_n(ch, count as usize))
            .collect()
    }
}

impl super::Solution for Solution {
    fn remove_duplicates(s: String, k: i32) -> String {
        Self::remove_duplicates(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
