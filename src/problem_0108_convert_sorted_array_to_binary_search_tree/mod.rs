pub mod recursive;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[-10, -3, 0, 5, 9] as &[_],
            &[Some(0), Some(-3), Some(9), Some(-10), None, Some(5)] as &[_],
        )];

        for (nums, expected) in test_cases {
            assert_eq!(
                S::sorted_array_to_bst(nums.to_vec()),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
