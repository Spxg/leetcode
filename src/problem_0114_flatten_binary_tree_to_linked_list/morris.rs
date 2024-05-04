pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root.clone();
        while let Some(node) = root.take() {
            let mut node = node.borrow_mut();
            if let Some(mut right_leaf) = node.left.clone() {
                loop {
                    let right = right_leaf.borrow().right.clone();
                    if let Some(right) = right {
                        right_leaf = right;
                    } else {
                        break;
                    }
                }
                let mut right_leaf = right_leaf.borrow_mut();
                right_leaf.right = node.right.take();
                node.right = node.left.take();
            }
            root.clone_from(&node.right);
        }
    }
}

impl super::Solution for Solution {
    fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::flatten(root);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
