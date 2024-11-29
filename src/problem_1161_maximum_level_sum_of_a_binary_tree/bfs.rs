pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result: Option<(i32, i32)> = None;
        let mut queue = VecDeque::from([root.unwrap()]);

        let mut level = 1;
        while !queue.is_empty() {
            let mut sum = 0;
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    let mut node = node.borrow_mut();
                    sum += node.val;
                    if let Some(x) = node.left.take() {
                        queue.push_back(x);
                    }
                    if let Some(x) = node.right.take() {
                        queue.push_back(x);
                    }
                }
            }
            if result.is_none_or(|x| x.1 < sum) {
                result = Some((level, sum));
            }
            level += 1;
        }

        result.unwrap().0
    }
}

impl super::Solution for Solution {
    fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_level_sum(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
