use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

pub type TreeNodeI32 = crate::treenode::TreeNode<i32>;

// Definition for a binary tree node.
#[derive(PartialEq, Eq)]
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

        // save root node
        let root = nodes[0].clone();

        nodes.reverse();

        let mut previous_level = Vec::new();
        let mut next_level = Vec::new();

        previous_level.push(nodes.pop().unwrap_or(None));

        while !&nodes.is_empty() {
            for n in &previous_level {
                if let Some(nn) = n {
                    if let Some(left) = nodes.pop() {
                        nn.as_ref().borrow_mut().left = left;
                        next_level.push(nn.as_ref().borrow().left.clone());
                    }

                    if let Some(right) = nodes.pop() {
                        nn.as_ref().borrow_mut().right = right;
                        next_level.push(nn.as_ref().borrow().right.clone());
                    }
                }
            }

            std::mem::swap(&mut previous_level, &mut next_level);
            next_level.clear();
        }

        root
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

impl<T: Copy + std::fmt::Display> std::fmt::Display for TreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let root = Rc::new(RefCell::new(Self {
            val: self.val,
            left: self.left.clone(),
            right: self.right.clone(),
        }));

        let mut previous_level = Vec::<Option<Rc<RefCell<Self>>>>::new();
        let mut next_level = Vec::<Option<Rc<RefCell<Self>>>>::new();

        let mut elements = Vec::<String>::new();

        previous_level.push(Some(root));

        while !&previous_level.is_empty() {
            for n in &previous_level {
                if let Some(nn) = n {
                    elements.push(format!("{}", nn.as_ref().borrow().val));
                    next_level.push(nn.as_ref().borrow().left.clone());
                    next_level.push(nn.as_ref().borrow().right.clone());
                } else {
                    elements.push("null".into());
                }
            }

            std::mem::swap(&mut previous_level, &mut next_level);
            next_level.clear();
        }

        // remove terminal 'null' pairs
        while elements.ends_with(&["null".into()]) {
            elements.pop();
        }

        write!(f, "[{}]", elements.join(", "))
    }
}

impl<T: Copy + std::fmt::Display> std::fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests_treenode {
    use crate::treenode::TreeNode;

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

    #[test]
    fn integer_tree_without_leftmost_node() {
        assert_eq!(
            TreeNode::from_vec_str("[3,0,4,null,2,null,null,1]"),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: None,
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            })))
        );
    }

    #[test]
    fn display_one_node() {
        let tree = TreeNode::<i32>::from_vec_str("[1]");

        assert_eq!(format!("{}", tree.unwrap().clone().borrow()), "[1]");
    }

    #[test]
    fn display_integer_three_nodes_with_none_from_vec_str() {
        let tree = TreeNode::<i32>::from_vec_str("[1, null, 3]");

        assert_eq!(
            format!("{}", tree.unwrap().clone().borrow()),
            "[1, null, 3]"
        );
    }

    #[test]
    fn display_integer_tree_without_leftmost_node() {
        let tree = TreeNode::<i32>::from_vec_str("[3,0,4,null,2,null,null,1]");
        assert_eq!(
            format!("{}", tree.unwrap().clone().borrow()),
            "[3, 0, 4, null, 2, null, null, 1]"
        );
    }

    // #[test]
    #[allow(dead_code)]
    fn debug_one_node() {
        let tree = TreeNode::<i32>::from_vec_str("[1]");

        assert_eq!(format!("{:?}", tree.unwrap().clone().borrow()), "1");
    }
}
