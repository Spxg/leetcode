pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut result = 0;
        let mut people = people;
        people.sort_unstable_by(|a, b| b.cmp(a));
        let mut in_boat = vec![false; people.len()];
        for idx in 0..people.len() {
            if in_boat[idx] {
                continue;
            }
            if people[idx] < limit {
                let offset = idx + 1;
                let slice = &people[offset..];
                if let Some(pos) = (slice.partition_point(|&x| x > limit - people[idx])
                    ..slice.len())
                    .find(|&x| !in_boat[offset + x])
                {
                    in_boat[offset + pos] = true;
                }
            }
            result += 1;
            in_boat[idx] = true;
        }
        result
    }
}

impl super::Solution for Solution {
    fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        Self::num_rescue_boats(people, limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
