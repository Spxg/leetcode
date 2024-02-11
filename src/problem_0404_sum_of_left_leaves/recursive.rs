pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn hepler(node: &Rc<RefCell<TreeNode>>, is_left: bool, result: &mut i32) {
            match (&node.borrow().left, &node.borrow().right) {
                (None, None) if is_left => *result += node.borrow().val,
                (None, Some(right)) => hepler(right, false, result),
                (Some(left), None) => hepler(left, true, result),
                (Some(left), Some(right)) => {
                    hepler(left, true, result);
                    hepler(right, false, result);
                }
                _ => (),
            }
        }
        let mut result = 0;
        if let Some(node) = root {
            hepler(&node, false, &mut result);
        }
        result
    }
}

impl super::Solution for Solution {
    fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_of_left_leaves(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
