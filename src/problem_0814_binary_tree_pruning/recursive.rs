pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(node: &RefCell<TreeNode>) -> bool {
            let (left, right) = match (&node.borrow().left, &node.borrow().right) {
                (None, None) => (true, true),
                (Some(left), None) => (helper(left), true),
                (None, Some(right)) => (true, helper(right)),
                (Some(left), Some(right)) => (helper(left), helper(right)),
            };
            if left {
                node.borrow_mut().left.take();
            }
            if right {
                node.borrow_mut().right.take();
            }
            left && right && node.borrow().val == 0
        }

        root.and_then(|node| (!helper(&node)).then_some(node))
    }
}

impl super::Solution for Solution {
    fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::prune_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
