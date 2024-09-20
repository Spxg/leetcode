pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(root: &Rc<RefCell<TreeNode>>, val: i32) -> i32 {
            let mut root = root.borrow_mut();
            root.val += root.right.as_ref().map_or(val, |node| helper(node, val));
            root.left
                .as_ref()
                .map_or(root.val, |node| helper(node, root.val))
        }
        root.as_ref().map(|x| helper(x, 0));
        root
    }
}

impl super::Solution for Solution {
    fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bst_to_gst(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
