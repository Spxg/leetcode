pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::from([root]);

        while !queue.is_empty() {
            let mut nodes = std::mem::take(&mut queue);
            while let Some(node) = nodes.pop_front() {
                if let Some(node) = node {
                    queue.push_back(node.borrow().left.clone());
                    queue.push_back(node.borrow().right.clone());
                } else if nodes.iter().chain(queue.iter()).any(Option::is_some) {
                    return false;
                }
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_complete_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
