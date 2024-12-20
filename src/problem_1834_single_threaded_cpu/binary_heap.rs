pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        #[derive(Eq, PartialEq)]
        struct Helper(i32, i32, usize);

        impl Ord for Helper {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                (self.1, self.2).cmp(&(other.1, other.2)).reverse()
            }
        }

        impl PartialOrd for Helper {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut result = Vec::with_capacity(tasks.len());
        let mut heap = BinaryHeap::<Helper>::with_capacity(tasks.len());
        let mut tasks = tasks
            .into_iter()
            .enumerate()
            .map(|(idx, nums)| (nums[0], nums[1], idx))
            .collect::<Vec<_>>();
        tasks.sort_unstable();

        let mut current = 0;

        'task_for: for task in tasks {
            while task.0 > current {
                if let Some(helper) = heap.pop() {
                    result.push(helper.2 as _);
                    current += helper.1;
                } else {
                    result.push(task.2 as _);
                    current = task.0 + task.1;
                    continue 'task_for;
                }
            }
            heap.push(Helper(task.0, task.1, task.2));
        }

        while let Some(helper) = heap.pop() {
            result.push(helper.2 as _);
        }

        result
    }
}

impl super::Solution for Solution {
    fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        Self::get_order(tasks)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
