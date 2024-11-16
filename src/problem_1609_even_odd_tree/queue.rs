pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::from([root.unwrap()]);
        for level in 0.. {
            let mut prev = if level % 2 == 0 { 0 } else { i32::MAX };
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let mut node_mut = node.borrow_mut();
                let val = node_mut.val;

                if level % 2 == 0 && (val % 2 == 0 || val <= prev)
                    || level % 2 != 0 && (val % 2 != 0 || val >= prev)
                {
                    return false;
                }
                prev = val;

                if let Some(left) = node_mut.left.take() {
                    queue.push_back(left);
                }

                if let Some(right) = node_mut.right.take() {
                    queue.push_back(right);
                }
            }
            if queue.is_empty() {
                break;
            }
        }
        true
    }
}

impl super::Solution for Solution {
    fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_even_odd_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
