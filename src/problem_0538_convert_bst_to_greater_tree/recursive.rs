use crate::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(root: &Rc<RefCell<TreeNode>>, val: i32) -> i32 {
            let mut root = root.borrow_mut();
            root.val += root.right.as_ref().map_or(val, |right| helper(right, val));
            root.left
                .as_ref()
                .map_or(root.val, |left| helper(left, root.val))
        }
        if let Some(root) = &root {
            helper(root, 0);
        }
        root
    }
}

impl super::Solution for Solution {
    fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::convert_bst(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
