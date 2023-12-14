pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helpr(root: &Rc<RefCell<TreeNode>>, val: i32) -> i32 {
            let val = val * 10 + root.borrow().val;
            match (&root.borrow().left, &root.borrow().right) {
                (None, None) => val,
                (Some(node), None) | (None, Some(node)) => helpr(node, val),
                (Some(left), Some(right)) => helpr(left, val) + helpr(right, val),
            }
        }
        root.map_or(0, |node| helpr(&node, 0))
    }
}

impl super::Solution for Solution {
    fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_numbers(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
