pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = root {
                helper(&node.borrow().left);
                helper(&node.borrow().right);
                let node_mut = &mut *node.borrow_mut();
                std::mem::swap(&mut node_mut.left, &mut node_mut.right);
            }
        }
        helper(&root);
        root
    }
}

impl super::Solution for Solution {
    fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
