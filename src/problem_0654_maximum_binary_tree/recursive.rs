use crate::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let (idx, &max) = nums.iter().enumerate().max_by_key(|x| x.1)?;
            Some(Rc::new(RefCell::new(TreeNode {
                val: max,
                left: helper(&nums[0..idx]),
                right: helper(&nums[idx + 1..]),
            })))
        }
        helper(&nums)
    }
}

impl super::Solution for Solution {
    fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_maximum_binary_tree(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
