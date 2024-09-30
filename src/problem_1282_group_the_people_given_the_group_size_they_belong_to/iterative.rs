pub struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut groups: Vec<Vec<i32>> = vec![vec![]; 500];

        for (nth, size) in group_sizes.into_iter().enumerate() {
            let n = size as usize;
            groups[n - 1].push(nth as i32);
            if groups[n - 1].len() == n {
                result.push(std::mem::take(&mut groups[n - 1]));
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        Self::group_the_people(group_sizes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
