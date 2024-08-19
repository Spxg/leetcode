pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let (mut idx1, mut idx2) = (0, 0);

        while idx1 < first_list.len() && idx2 < second_list.len() {
            let f = &first_list[idx1];
            let s = &second_list[idx2];
            let (start, end) = (f[0].max(s[0]), f[1].min(s[1]));
            if start <= end {
                result.push(vec![start, end]);
            }
            if f[1] < s[1] {
                idx1 += 1;
            } else {
                idx2 += 1;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        Self::interval_intersection(first_list, second_list)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
