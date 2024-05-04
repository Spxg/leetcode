pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        let mut root = root;
        let mut count = 1;

        loop {
            if let Some(node) = root.take() {
                root.clone_from(&node.borrow().left);
                stack.push(node);
            } else {
                let node = stack.pop().unwrap();
                if count == k {
                    return node.borrow().val;
                }
                count += 1;
                root.clone_from(&node.borrow().right);
            }
        }
    }
}

impl super::Solution for Solution {
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::kth_smallest(root, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
