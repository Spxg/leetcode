pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(node: &Rc<RefCell<TreeNode>>, target: i32) -> bool {
            let mut node = node.borrow_mut();
            if let Some(left) = &node.left {
                if helper(left, target) {
                    node.left.take();
                }
            }
            if let Some(right) = &node.right {
                if helper(right, target) {
                    node.right.take();
                }
            }
            node.left.is_none() && node.right.is_none() && node.val == target
        }
        let root = root.unwrap();
        (!helper(&root, target)).then_some(root)
    }
}

impl super::Solution for Solution {
    fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::remove_leaf_nodes(root, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
