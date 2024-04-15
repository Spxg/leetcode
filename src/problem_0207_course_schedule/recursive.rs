pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        fn helper(course: usize, map: &[Vec<i32>], cache: &mut [bool], deps: &mut [bool]) -> bool {
            if cache[course] {
                return true;
            }
            if deps[course] {
                return false;
            }

            deps[course] = true;
            for &course in &map[course] {
                if !helper(course as usize, map, cache, deps) {
                    return false;
                }
            }
            deps[course] = false;

            cache[course] = true;
            true
        }

        let num_courses = num_courses as usize;
        let mut map = vec![vec![]; num_courses];
        let mut deps = vec![false; num_courses];
        let mut cache = vec![false; num_courses];

        prerequisites
            .into_iter()
            .for_each(|x| map[x[0] as usize].push(x[1]));

        (0..num_courses).all(|course| helper(course, &map, &mut cache, &mut deps))
    }
}

impl super::Solution for Solution {
    fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        Self::can_finish(num_courses, prerequisites)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
