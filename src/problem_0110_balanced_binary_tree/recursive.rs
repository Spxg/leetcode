pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            if let Some(node) = node {
                let left = helper(&node.borrow().left)?;
                let right = helper(&node.borrow().right)?;
                ((left - right).abs() <= 1).then(|| left.max(right) + 1)
            } else {
                Some(0)
            }
        }
        helper(&root).is_some()
    }
}

impl super::Solution for Solution {
    fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
