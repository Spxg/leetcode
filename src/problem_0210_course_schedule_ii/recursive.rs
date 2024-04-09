pub struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        fn helper(
            idx: usize,
            map: &[Vec<i32>],
            result: &mut Vec<i32>,
            added: &mut [bool],
            dep_set: &mut [bool],
        ) -> bool {
            if added[idx] {
                return true;
            }
            if dep_set[idx] {
                return false;
            }

            dep_set[idx] = true;
            for &dep in &map[idx] {
                if !helper(dep as usize, map, result, added, dep_set) {
                    return false;
                }
            }
            dep_set[idx] = false;

            added[idx] = true;
            result.push(idx as i32);
            true
        }

        let num_courses = num_courses as usize;
        let mut map: Vec<Vec<i32>> = vec![vec![]; num_courses];
        let mut result = Vec::with_capacity(num_courses);
        let mut added = vec![false; num_courses];
        let mut dep_set = vec![false; num_courses];

        prerequisites
            .into_iter()
            .for_each(|x| map[x[0] as usize].push(x[1]));

        if (0..num_courses).all(|idx| helper(idx, &map, &mut result, &mut added, &mut dep_set)) {
            result
        } else {
            vec![]
        }
    }
}

impl super::Solution for Solution {
    fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_order(num_courses, prerequisites)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
