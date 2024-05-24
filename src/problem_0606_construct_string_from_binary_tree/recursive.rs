use crate::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn helper(node: &Rc<RefCell<TreeNode>>) -> String {
            let val = node.borrow().val;
            match (&node.borrow().left, &node.borrow().right) {
                (None, None) => format!("{val}"),
                (None, Some(right)) => format!("{val}()({})", helper(right)),
                (Some(left), None) => format!("{val}({})", helper(left)),
                (Some(left), Some(right)) => format!("{val}({})({})", helper(left), helper(right)),
            }
        }
        helper(&root.unwrap())
    }
}

impl super::Solution for Solution {
    fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::tree2str(t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
