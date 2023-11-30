// https://codereview.stackexchange.com/questions/266554/bst-implementation-in-rust
use std::ops::Add;

type TreeNode<T> = Box<Node<T>>;

pub struct Tree<T> {
    root: Option<TreeNode<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<TreeNode<T>>,
    right: Option<TreeNode<T>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, node: TreeNode<T>) {
        if self.value > node.value {
            if let Some(ref mut left) = self.left {
                left.insert(node);
            } else {
                self.left = Some(node);
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(node);
            } else {
                self.right = Some(node);
            }
        }
    }
}

impl<T: Ord + Add<Output = T> + Default + Copy> Tree<T> {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: T) -> &mut Self {
        let node = Box::new(Node::new(value));
        if let Some(ref mut root) = self.root {
            root.insert(node);
        } else {
            self.root = Some(node);
        }
        self
    }

    fn sum_tree(&self) -> T {
        let mut sum: T = Default::default();
        if self.root.is_none() {
            sum
        } else {
            let mut stack = vec![self.root.as_ref().unwrap()];

            while stack.len() > 0 {
                if let Some(n) = stack.pop() {
                    sum = sum + n.value;

                    if n.left.is_some() {
                        stack.push(n.left.as_ref().unwrap())
                    }
                    if n.right.is_some() {
                        stack.push(n.right.as_ref().unwrap())
                    }
                }
            }

            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_tree() {
        let tree: Tree<u32> = Tree::new();
        assert!(tree.root.is_none())
    }

    #[test]
    fn insert_root() {
        let mut tree: Tree<u32> = Tree::new();
        assert!(tree.root.is_none());

        tree.insert(1);
        assert_eq!(tree.root.as_ref().unwrap().value, 1);
        assert_eq!(tree.sum_tree(), 1);
    }

    #[test]
    fn insert_1_2() {
        let mut tree: Tree<u32> = Tree::new();
        tree.insert(1);
        tree.insert(2);
        // https://stackoverflow.com/a/25298034/927687
        // use as_ref to convert Option<T> to Option<&T>, which avoid move unlike unwrap
        let root = tree.root.unwrap();

        assert_eq!(root.value, 1);
        let right = root.right.as_ref().unwrap();
        assert_eq!(right.value, 2);
    }

    #[test]
    fn insert_2_1_3() {
        let mut tree: Tree<u32> = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(3);

        let root = tree.root.as_ref().unwrap();

        assert_eq!(root.value, 2);
        let right = root.right.as_ref().unwrap();
        assert_eq!(right.value, 3);
        let left = root.left.as_ref().unwrap();
        assert_eq!(left.value, 1);

        assert_eq!(tree.sum_tree(), 6);
    }

    #[test]
    fn insert_1_2_3() {
        let mut tree: Tree<u32> = Tree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        let root = tree.root.unwrap();

        assert_eq!(root.value, 1);
        let right = root.right.as_ref().unwrap();
        assert_eq!(right.value, 2);
        let right2 = right.right.as_ref().unwrap();
        assert_eq!(right2.value, 3);
    }
}

