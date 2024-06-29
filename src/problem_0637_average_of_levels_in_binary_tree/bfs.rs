pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        loop {
            if queue.is_empty() {
                break;
            }
            let size = queue.len();
            let mut sum = 0i64;
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                if let Some(left) = node.borrow_mut().left.take() {
                    queue.push_back(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    queue.push_back(right);
                }
                sum += i64::from(node.borrow().val);
            }
            #[allow(clippy::cast_precision_loss)]
            result.push(sum as f64 / size as f64);
        }
        result
    }
}

impl super::Solution for Solution {
    fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        Self::average_of_levels(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
