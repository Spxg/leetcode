pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some((&first, rest)) = preorder.split_first() {
                let postorder = &postorder[0..postorder.len() - 1];
                let mut node = TreeNode::new(first);
                if let Some(pos) = rest
                    .first()
                    .and_then(|&next| postorder.iter().position(|&x| x == next))
                {
                    node.left = helper(&rest[0..=pos], &postorder[0..=pos]);
                    node.right = helper(&rest[pos + 1..], &postorder[pos + 1..]);
                }
                Some(Rc::new(RefCell::new(node)))
            } else {
                None
            }
        }
        helper(&preorder, &postorder)
    }
}

impl super::Solution for Solution {
    fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_from_pre_post(preorder, postorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
