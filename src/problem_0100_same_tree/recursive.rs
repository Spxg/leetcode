use std::cell::RefCell;
use std::rc::Rc;

use crate::data_structures::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn helper(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (Some(p), Some(q)) if p.borrow().val == q.borrow().val => {
                    helper(&p.borrow().left, &q.borrow().left)
                        && helper(&p.borrow().right, &q.borrow().right)
                }
                (None, None) => true,
                _ => false,
            }
        }
        helper(&p, &q)
    }
}

impl super::Solution for Solution {
    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_same_tree(p, q)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
