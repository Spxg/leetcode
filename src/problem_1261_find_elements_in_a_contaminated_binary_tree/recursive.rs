pub struct Solution;

use crate::data_structures::TreeNode;

use std::{cell::RefCell, collections::HashSet, rc::Rc};

struct FindElements {
    set: HashSet<i32>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn helper(node: &Rc<RefCell<TreeNode>>, val: i32, set: &mut HashSet<i32>) {
            set.insert(val);
            if let Some(left) = &node.borrow().left {
                helper(left, val * 2 + 1, set);
            }
            if let Some(right) = &node.borrow().right {
                helper(right, val * 2 + 2, set);
            }
        }
        let mut set = HashSet::new();
        helper(&root.unwrap(), 0, &mut set);
        Self { set }
    }

    fn find(&self, target: i32) -> bool {
        self.set.contains(&target)
    }
}

impl super::FindElements for FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self::new(root)
    }

    fn find(&self, target: i32) -> bool {
        self.find(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::FindElements>();
    }
}
