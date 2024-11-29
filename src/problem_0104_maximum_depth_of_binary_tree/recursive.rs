pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        #[allow(clippy::ref_option)]
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            node.as_ref().map_or(depth, |node| {
                helper(&node.borrow().left, depth + 1).max(helper(&node.borrow().right, depth + 1))
            })
        }
        helper(&root, 0)
    }
}

impl super::Solution for Solution {
    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
