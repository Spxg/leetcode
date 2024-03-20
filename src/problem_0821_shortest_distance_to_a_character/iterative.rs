pub struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut result = Vec::with_capacity(s.len());
        let mut prev = None;
        for (idx, ch) in (0..).zip(s.chars()) {
            if ch == c {
                if let Some(prev) = prev {
                    let distance = idx - prev - 1;
                    (1..=distance / 2).for_each(|x| result.push(x));
                    if distance % 2 != 0 {
                        result.push(distance / 2 + 1);
                    }
                    (1..=distance / 2).rev().for_each(|x| result.push(x));
                } else {
                    (1..=idx).rev().for_each(|x| result.push(x));
                }
                result.push(0);
                prev = Some(idx);
            } else if idx == s.len() as i32 - 1 {
                if let Some(prev) = prev {
                    (1..=idx - prev).for_each(|x| result.push(x));
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        Self::shortest_to_char(s, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
