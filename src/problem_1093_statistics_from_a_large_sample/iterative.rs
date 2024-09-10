pub struct Solution;

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let counts = count.iter().sum::<i32>();

        let (idx1, idx2) = if counts % 2 == 0 {
            (counts / 2, counts / 2 + 1)
        } else {
            ((counts + 1) / 2, (counts + 1) / 2)
        };

        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut mode = (0, 0);
        let mut start = 1;
        let mut median = 0;
        let mut sum = 0;

        for (num, &count) in (0..).zip(count.iter()) {
            if (start..start + count).contains(&idx1) {
                median += num;
            }
            if (start..start + count).contains(&idx2) {
                median += num;
            }

            if count != 0 {
                mode = mode.max((count, num));
                min = min.min(num);
                max = max.max(num);
                sum += num as u64 * count as u64;
            }

            start += count;
        }

        let median = f64::from(median) / 2.0;
        #[allow(clippy::pedantic)]
        let mean = (sum as f64) / f64::from(counts);

        vec![
            f64::from(min),
            f64::from(max),
            mean,
            median,
            f64::from(mode.1),
        ]
    }
}

impl super::Solution for Solution {
    fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        Self::sample_stats(count)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
