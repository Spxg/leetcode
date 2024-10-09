pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut queue = VecDeque::from([root.unwrap()]);

        while !queue.is_empty() {
            let mut sum = 0;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let mut node_mut = node.borrow_mut();
                sum += node_mut.val;
                if let Some(left) = node_mut.left.take() {
                    queue.push_back(left);
                }
                if let Some(right) = node_mut.right.take() {
                    queue.push_back(right);
                }
            }
            result = sum;
        }

        result
    }
}

impl super::Solution for Solution {
    fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::deepest_leaves_sum(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
