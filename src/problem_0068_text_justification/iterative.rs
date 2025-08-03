pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = vec![];
        let mut start = 0;
        let mut size = 0;
        let max_width = max_width as usize;

        for idx in 0..words.len() {
            let word_len = words[idx].len();
            size += word_len;
            match size.cmp(&max_width) {
                std::cmp::Ordering::Less => size += 1,
                std::cmp::Ordering::Equal => {
                    result.push(words[start..=idx].join(" "));
                    start = idx + 1;
                    size = 0;
                }
                std::cmp::Ordering::Greater => {
                    let ws = idx - start;
                    let space = max_width - (size - word_len) + ws;
                    let slot = if ws == 1 { 1 } else { ws - 1 };
                    let base = space / slot;
                    let rest = space % slot;

                    let mut iter = std::iter::repeat_n(base + 1, rest)
                        .chain(std::iter::repeat_n(base, slot - rest));

                    let mut part = String::with_capacity(max_width);
                    for s in &words[start..idx] {
                        part.push_str(s);
                        if let Some(size) = iter.next() {
                            (0..size).for_each(|_| part.push(' '));
                        }
                    }
                    result.push(part);
                    start = idx;
                    size = words[idx].len() + 1;
                }
            }
        }

        if start < words.len() {
            let ws = words.len() - start;
            let space = max_width - size + ws;
            let slot = ws - 1;
            let mut part = words[start..].join(" ");
            (0..space - slot).for_each(|_| part.push(' '));
            result.push(part);
        }

        result
    }
}

impl super::Solution for Solution {
    fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        Self::full_justify(words, max_width)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
