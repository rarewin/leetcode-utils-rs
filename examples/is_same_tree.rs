use leetcode_utils_rs::TreeNodeI32 as TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            true
        } else if let (Some(pn), Some(qn)) = (p, q) {
            pn.as_ref().borrow().val == qn.as_ref().borrow().val
                && Solution::is_same_tree(
                    pn.as_ref().borrow().left.clone(),
                    qn.as_ref().borrow().left.clone(),
                )
                && Solution::is_same_tree(
                    pn.as_ref().borrow().right.clone(),
                    qn.as_ref().borrow().right.clone(),
                )
        } else {
            false
        }
    }
}

fn main() {
    assert_eq!(
        Solution::is_same_tree(TreeNode::from_vec_str("[1]"), TreeNode::from_vec_str("[1]")),
        true
    );

    assert_eq!(
        Solution::is_same_tree(
            TreeNode::from_vec_str("[1, 2, 3]"),
            TreeNode::from_vec_str("[1, 2, 3]")
        ),
        true
    );

    assert_eq!(
        Solution::is_same_tree(
            TreeNode::from_vec_str("[1, 2]"),
            TreeNode::from_vec_str("[1, null, 2]")
        ),
        false
    );

    assert_eq!(
        Solution::is_same_tree(
            TreeNode::from_vec_str("[1, 2, 1]"),
            TreeNode::from_vec_str("[1, 1, 2]")
        ),
        false
    );
}
