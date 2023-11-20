pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn hepler(node: Rc<RefCell<TreeNode>>, target: i32, sum: i32) -> bool {
            match (node.borrow().left.clone(), node.borrow().right.clone()) {
                (None, None) => target == sum + node.borrow().val,
                (None, Some(right)) => hepler(right, target, sum + node.borrow().val),
                (Some(left), None) => hepler(left, target, sum + node.borrow().val),
                (Some(left), Some(right)) => {
                    hepler(right, target, sum + node.borrow().val) || hepler(left, target, sum + node.borrow().val)
                }
            }
        }

        root.map_or(false, |node| hepler(node, target_sum, 0))
    }
}

impl super::Solution for Solution {
    fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Self::has_path_sum(root, sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
