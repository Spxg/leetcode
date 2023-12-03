pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            (!nums.is_empty()).then(|| {
                let mid = nums.len() / 2;
                Rc::new(RefCell::new(TreeNode {
                    val: nums[mid],
                    left: helper(&nums[0..mid]),
                    right: helper(&nums[mid + 1..]),
                }))
            })
        }
        helper(&nums)
    }
}

impl super::Solution for Solution {
    fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sorted_array_to_bst(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
