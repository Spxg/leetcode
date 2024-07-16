pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result = vec![];

        let mut chs = [(-1, -1); 26];
        for (i, byte) in (0..).zip(s.bytes()) {
            let idx = (byte - b'a') as usize;
            if chs[idx].0 == -1 {
                chs[idx].0 = i;
            }
            chs[idx].1 = i;
        }
        chs.sort_unstable();
        let offset = chs.into_iter().position(|x| x.0 == 0).unwrap();
        let mut prev = chs[offset];
        for current in chs.into_iter().skip(offset + 1) {
            match prev.1.cmp(&current.0) {
                std::cmp::Ordering::Less => {
                    result.push(prev.1 - prev.0 + 1);
                    prev = current;
                }
                std::cmp::Ordering::Equal => unreachable!(),
                std::cmp::Ordering::Greater => prev.1 = current.1.max(prev.1),
            }
        }
        result.push(prev.1 - prev.0 + 1);
        result
    }
}

impl super::Solution for Solution {
    fn partition_labels(s: String) -> Vec<i32> {
        Self::partition_labels(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
