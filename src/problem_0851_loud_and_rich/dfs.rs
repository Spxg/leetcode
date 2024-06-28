pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        fn helper(
            person: usize,
            richer_map: &HashMap<usize, Vec<usize>>,
            quiet: &[i32],
            result: &mut [i32],
        ) -> (i32, usize) {
            if result[person] != -1 {
                let person = result[person] as usize;
                return (quiet[person], person);
            }

            let mut min = (quiet[person], person);
            if let Some(richer) = richer_map.get(&person) {
                for &person in richer {
                    let child = helper(person, richer_map, quiet, result);
                    min = min.min(child);
                }
            }
            result[person] = min.1 as i32;
            min
        }

        let mut result = vec![-1; quiet.len()];
        let mut richer_map = HashMap::<usize, Vec<usize>>::with_capacity(quiet.len());
        for richer in richer {
            richer_map
                .entry(richer[1] as usize)
                .and_modify(|x| x.push(richer[0] as usize))
                .or_insert_with(|| vec![richer[0] as usize]);
        }
        for person in 0..quiet.len() {
            helper(person, &richer_map, &quiet, &mut result);
        }
        result
    }
}

impl super::Solution for Solution {
    fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        Self::loud_and_rich(richer, quiet)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
