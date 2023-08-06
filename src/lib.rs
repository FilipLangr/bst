use std::mem;


impl<'a, T: PartialOrd + PartialEq> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = BSTRefIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BSTRefIter::new(self)
    }
}

enum StackRefMember<'a, T: PartialOrd + PartialEq> {
    Node(&'a Box<Node<T>>),
    Visited(&'a T),
}

pub struct BSTRefIter<'a, T: PartialOrd + PartialEq> {
    stack: Vec<StackRefMember<'a, T>>,
}

impl <'a, T: PartialOrd + PartialEq> BSTRefIter<'a, T> {
    fn new(bst: &BST<T>) -> BSTRefIter<T> {
        let mut stack = Vec::new();
        if let Some(root) = &bst.root {
            stack.push(StackRefMember::Node(root));
        }
        BSTRefIter{ stack }
    }
}

impl<'a, T: PartialOrd + PartialEq> Iterator for BSTRefIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        while let Some(stack_member) = self.stack.pop() {
            match stack_member {
                StackRefMember::Visited(value) => {
                    return Some(value);
                },
                StackRefMember::Node(node) => {
                    if let Some(right) = &node.right {
                        self.stack.push(StackRefMember::Node(right));
                    }
                    self.stack.push(StackRefMember::Visited(&node.value) );
                    if let Some(left) = &node.left {
                        self.stack.push(StackRefMember::Node(left));
                    }
                },
            }
        }
        None
    }
}

impl<T: PartialOrd + PartialEq> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = BSTConsumingIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        BSTConsumingIter::new(self)
    }
}

pub struct BSTConsumingIter<T: PartialOrd + PartialEq> {
    stack: Vec<Box<Node<T>>>,
}

impl<T: PartialOrd + PartialEq> BSTConsumingIter<T> {
    fn new(bst: BST<T>) -> BSTConsumingIter<T> {
        let mut stack = Vec::new();
        if let Some(root) = bst.root {
            stack.push(root);
        }
        BSTConsumingIter { stack }
    }
}

impl<T: PartialOrd + PartialEq> Iterator for BSTConsumingIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        while let Some(mut node) = self.stack.pop() {
            match node.left.take() {
                Some(left) => {
                    self.stack.push(node);
                    self.stack.push(left);
                },
                None => {
                    if let Some(right) = node.right.take() {
                        self.stack.push(right);
                    };
                    return Some(node.value);
                },
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct BST<T: PartialOrd + PartialEq> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + PartialEq> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }
    pub fn insert(&mut self, value: T) {
        match &mut self.root {
            None => self.root = Some(Box::new(Node::leaf(value))),
            Some(root_node) => root_node.insert(value),
        };
    }
    pub fn contains(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.contains(value))
    }
    pub fn delete(&mut self, value: T) {
        match &mut self.root {
            None => {}
            Some(root) if value == root.value => {
                Node::delete_by_node(&mut self.root);
            }
            Some(root) => {
                root.delete(value)
            }
        }
    }
    pub fn iter(&self) -> BSTRefIter<T> {
        self.into_iter()
    }
}

#[derive(Debug, PartialEq)]
struct Node<T: PartialOrd + PartialEq> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + PartialEq> Node<T> {
    fn add_left(&mut self, value: T) {
        self.left = Some(Box::new(Node::leaf(value)))
    }
    fn add_right(&mut self, value: T) {
        self.right = Some(Box::new(Node::leaf(value)))
    }
    fn leaf(value: T) -> Node<T> {
        Node { value, right: None, left: None }
    }
    fn insert(&mut self, value: T) {
        if value < self.value {
            match &mut self.left {
                None => { self.add_left(value) }
                Some(boxed) => { boxed.insert(value) }
            }
        } else if value > self.value {
            match &mut self.right {
                None => { self.add_right(value) }
                Some(boxed) => { boxed.insert(value) }
            }
        };
    }
    fn contains(&self, value: T) -> bool {
        if value == self.value { true } else if value < self.value {
            self.left.as_ref().map_or(false, |left| left.contains(value))
        } else {
            self.right.as_ref().map_or(false, |right| right.contains(value))
        }
    }
    fn delete(&mut self, value: T) {
        match (&mut self.left, &mut self.right) {
            (_, Some(right)) if value == right.value => Node::delete_by_node(&mut self.right),
            (Some(left), _) if value == left.value => Node::delete_by_node(&mut self.left),
            (Some(left), _) if value < self.value => {
                left.delete(value)
            }
            (_, Some(right)) if value > self.value => {
                right.delete(value)
            }
            _ => {}
        };
    }
    fn delete_by_node(node: &mut Option<Box<Node<T>>>) {
        match node.take() {
            None => { return; }
            Some(to_delete) => match (to_delete.left, to_delete.right) {
                (None, None) => { node.take(); }
                (None, Some(right)) => { node.replace(right); }
                (Some(left), None) => { node.replace(left); }
                (Some(left), Some(mut right)) => {
                    let mut new_node = match right.delete_get_leftmost() {
                        None => right,
                        Some(mut new_node) => {
                            new_node.right = Some(right);
                            new_node
                        }
                    };
                    new_node.left = Some(left);
                    node.replace(new_node);
                }
            }
        }
    }
    fn delete_get_leftmost(&mut self) -> Option<Box<Node<T>>> {
        match &mut self.left {
            None => None,
            Some(left) => {
                match left.delete_get_leftmost() {
                    None => {
                        match left.right.take() {
                            None => self.left.take(),
                            Some(right) => {
                                Some(mem::replace(left, right))
                            }
                        }
                    }
                    otherwise => otherwise,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let bst = BST::new();
        assert_eq!(bst, BST { root: None });
        assert_ne!(bst, BST { root: Some(Box::new(Node { value: 11, left: None, right: None })) });
    }

    #[test]
    fn root_eq() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        assert_eq!(bst, BST { root: Some(Box::new(Node { value: 10, left: None, right: None })) });
        assert_ne!(bst, BST { root: Some(Box::new(Node { value: 11, left: None, right: None })) });
        assert_ne!(bst, BST { root: Some(Box::new(Node { value: 11, left: Some(Box::new(Node { value: 10, left: None, right: None })), right: None })) });
    }

    #[test]
    fn insert() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        bst.insert(18);
        bst.insert(11);
        bst.insert(13);
        bst.insert(17);
        bst.insert(19);
        bst.insert(15);
        bst.insert(12);
        bst.insert(10);
        assert_eq!(
            bst,
            BST {
                root: Some(Box::new(Node {
                    value: 10,
                    left: Some(Box::new(Node { value: 5, left: None, right: None })),
                    right: Some(Box::new(Node {
                        value: 15,
                        left: Some(Box::new(Node {
                            value: 12,
                            left: Some(Box::new(Node {
                                value: 11,
                                left: None,
                                right:
                                None,
                            })),
                            right: Some(Box::new(Node { value: 13, left: None, right: None })),
                        })),
                        right: Some(Box::new(Node { value: 18, left: Some(Box::new(Node { value: 17, left: None, right: None })), right: Some(Box::new(Node { value: 19, left: None, right: None })) })),
                    })),
                }))
            }
        );
    }

    #[test]
    fn delete() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        bst.insert(18);
        bst.insert(11);
        bst.insert(13);
        bst.insert(17);
        bst.insert(19);

        bst.delete(10);
        bst.delete(11);
        bst.delete(5);
        bst.delete(18);
        assert_eq!(
            bst,
            BST {
                root: Some(Box::new(Node {
                    value: 12,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 15,
                        left: Some(Box::new(Node {
                            value: 13,
                            left: None,
                            right: None,
                        })),
                        right: Some(Box::new(Node {
                            value: 19,
                            left: Some(Box::new(Node {
                                value: 17,
                                left: None,
                                right: None,
                            })),
                            right: None,
                        })),
                    })),
                }))
            }
        );

        bst.delete(12);
        bst.delete(15);
        bst.delete(19);
        bst.delete(17);
        bst.delete(13);
        assert_eq!(bst, BST { root: None });
    }

    #[test]
    fn contains() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(10);
        bst.insert(7);

        assert!(bst.contains(10));
        assert!(bst.contains(5));
        assert!(bst.contains(7));
        assert!(!bst.contains(1313));

        bst.delete(10);
        bst.delete(7);
        bst.delete(1313);

        assert!(!bst.contains(10));
        assert!(bst.contains(5));
        assert!(!bst.contains(7));
        assert!(!bst.contains(1313));
        assert!(!bst.contains(1414));
    }

    #[test]
    fn iter() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        bst.insert(18);
        bst.insert(11);
        bst.insert(13);
        bst.insert(17);
        bst.insert(19);
        bst.insert(15);
        bst.insert(12);
        bst.insert(10);

        let sorted_values: Vec<i32> = vec![5, 10, 11, 12, 13, 15, 17, 18, 19];
        assert_eq!(sorted_values, (&bst).into_iter().map(|v| *v).collect::<Vec<i32>>());

        bst.delete(10);
        let sorted_values: Vec<i32> = vec![5, 11, 12, 13, 15, 17, 18, 19];
        assert_eq!(sorted_values, (&bst).into_iter().map(|v| *v).collect::<Vec<i32>>());

        bst.insert(14);
        let sorted_values: Vec<i32> = vec![5, 11, 12, 13, 14, 15, 17, 18, 19];
        assert_eq!(sorted_values, (&bst).into_iter().map(|v| *v).collect::<Vec<i32>>());
        assert_eq!(sorted_values, bst.into_iter().collect::<Vec<i32>>());
    }
}
