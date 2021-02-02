use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

pub type TreeNodeI32 = TreeNode<i32>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T: Copy> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Copy> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl<T: Copy + std::fmt::Debug + FromStr> TreeNode<T> {
    pub fn from_vec(tree: Vec<Option<T>>) -> Option<Rc<RefCell<Self>>> {
        if tree.is_empty() {
            return None;
        }

        let mut nodes = Vec::new();

        for node in tree.iter() {
            nodes.push(match node {
                Some(n) => Some(Rc::new(RefCell::new(TreeNode::new(*n)))),
                None => None,
            });
        }

        for i in 0..(tree.len() / 2) {
            if let Some(Some(p)) = nodes.get(i) {
                if let Some(Some(n)) = nodes.get(2 * (i + 1) - 1) {
                    p.clone().borrow_mut().left = Some(n.clone());
                }
                if let Some(Some(n)) = nodes.get(2 * (i + 1)) {
                    p.clone().borrow_mut().right = Some(n.clone());
                }
            }
        }

        nodes[0].clone()
    }

    pub fn from_vec_str(tree_str: &str) -> Option<Rc<RefCell<Self>>> {
        let tree_string = tree_str.trim();

        // string should be en_stringclosed by brackets
        let tree_string = tree_string.strip_prefix("[")?;
        let tree_string = tree_string.strip_suffix("]")?;

        let elements = tree_string.split(',').collect::<Vec<&str>>();

        let mut tree = Vec::new();

        for e in elements {
            tree.push(match e.trim() {
                "null" => None,
                _ => Some(e.trim().parse::<T>().ok()?),
            });
        }

        TreeNode::from_vec(tree)
    }
}

#[cfg(test)]
mod tests_treenode {
    use crate::TreeNode;

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn integer_single_node() {
        let tree = vec![Some(1)];

        assert_eq!(
            TreeNode::from_vec(tree),
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        );
    }

    #[test]
    fn integer_three_nodes() {
        let tree = vec![Some(1), Some(2), Some(3)];

        assert_eq!(
            TreeNode::from_vec(tree),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))
        );
    }

    #[test]
    fn integer_three_nodes_with_none() {
        let tree = vec![Some(1), None, Some(3)];

        assert_eq!(
            TreeNode::from_vec(tree),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))
        );
    }

    #[test]
    fn integer_seven_nodes() {
        let tree = vec![Some(1), Some(2), Some(3), None, Some(5), Some(6), Some(7)];

        assert_eq!(
            TreeNode::from_vec(tree),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            })))
        );
    }

    #[test]
    fn integer_single_node_from_vec_str() {
        assert_eq!(
            TreeNode::from_vec_str("[1]"),
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        );
    }

    #[test]
    fn integer_three_nodes_with_none_from_vec_str() {
        assert_eq!(
            TreeNode::from_vec_str("[1, null, 3]"),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))
        );
    }

    #[test]
    fn integer_two_nodes_from_vec_str() {
        assert_eq!(
            TreeNode::from_vec_str("[1, 3]"),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            })))
        );
    }
}
