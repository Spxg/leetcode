pub mod recursive;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(
            (&[9, 3, 15, 20, 7] as &[_], &[9, 15, 7, 20, 3] as &[_]),
            &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_],
        )];

        for ((inorder, postorder), expected) in test_cases {
            assert_eq!(
                S::build_tree(inorder.to_vec(), postorder.to_vec()),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
