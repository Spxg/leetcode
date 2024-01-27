pub struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut result = String::new();
        let mut feq = [('0', 0); 128];

        for ch in s.chars() {
            feq[ch as usize].0 = ch;
            feq[ch as usize].1 += 1;
        }

        feq.sort_unstable_by(|&(_, feq1), &(_, feq2)| feq2.cmp(&feq1));

        for (ch, counts) in feq {
            if counts == 0 {
                break;
            }
            (0..counts).for_each(|_| result.push(ch));
        }

        result
    }
}

impl super::Solution for Solution {
    fn frequency_sort(s: String) -> String {
        Self::frequency_sort(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
