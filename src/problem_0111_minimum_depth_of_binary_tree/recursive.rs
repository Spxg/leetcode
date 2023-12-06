pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Rc<RefCell<TreeNode>>, depth: i32) -> i32 {
            match (&root.borrow().left, &root.borrow().right) {
                (None, None) => depth,
                (None, Some(node)) | (Some(node), None) => helper(node, depth + 1),
                (Some(left), Some(right)) => helper(left, depth + 1).min(helper(right, depth + 1)),
            }
        }
        root.map_or(0, |root| helper(&root, 1))
    }
}

impl super::Solution for Solution {
    fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
