use std::cmp::max;
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
            None => self.root = Node::new_node(value, None, None),
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

    pub fn left_rotation_of_root(&mut self) { // TODO temp
        if let Some(root) = self.root.take() {
            let new_root = root.left_rotation();
            self.root = new_root;
        }
    }
    pub fn right_rotation_of_root(&mut self) { // TODO temp
        if let Some(root) = self.root.take() {
            let new_root = root.right_rotation();
            self.root = new_root;
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node<T: PartialOrd + PartialEq> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: u32,
    left_height: u32,
    right_height: u32,
}

impl<T: PartialOrd + PartialEq> Node<T> {
    fn add_left(&mut self, value: T) {
        self.left = Node::new_node(value, None, None)
    }
    fn add_right(&mut self, value: T) {
        self.right = Node::new_node(value, None, None)
    }
    fn new_node(value: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let mut new_node = Box::new(Node{ value: value, left: left, right: right, height: 0, left_height: 0, right_height: 0});
        new_node.update_heights();
        Some(new_node)
    }
    fn update_heights(&mut self) {
        self.left_height = self.left.as_ref().map_or(0, |v| v.height + 1);
        self.right_height = self.right.as_ref().map_or(0, |v| v.height + 1);
        self.height = max(self.left_height, self.right_height);
    }
    fn is_left_heavy(&self) -> bool {
        self.left_height > self.right_height
    }
    fn is_right_heavy(&self) -> bool {
        self.right_height > self.left_height
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
        self.update_heights();
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
        self.update_heights();
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
                    new_node.update_heights();
                    node.replace(new_node);
                }
            }
        }
    }
    fn delete_get_leftmost(&mut self) -> Option<Box<Node<T>>> {
        match &mut self.left {
            None => None,
            Some(left) => {
                let ret_val = match left.delete_get_leftmost() {
                    None => {
                        match left.right.take() {
                            None => self.left.take(),
                            Some(right) => {
                                Some(mem::replace(left, right))
                            }
                        }
                    }
                    otherwise => otherwise,
                };
                self.update_heights(); // TODO our tests do not cover this one missing, fix it
                ret_val
            }
        }
    }
    fn left_rotation(mut self) -> Option<Box<Node<T>>>{
        if let Some(mut right) = self.right.take() {
            self.right = right.left.take();
            self.update_heights();
            right.left = Some(Box::new(self));
            right.update_heights();
            return Some(right)
        }
        None
    }
    fn right_rotation(mut self) -> Option<Box<Node<T>>>{
        if let Some(mut left) = self.left.take() {
            self.left = left.right.take();
            self.update_heights();
            left.right = Some(Box::new(self));
            left.update_heights();
            return Some(left)
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let bst = BST::new();
        assert_eq!(bst, BST { root: None });
        assert_ne!(bst, BST { root: Node::new_node(11, None, None ) });
    }

    #[test]
    fn root_eq() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        assert_eq!(bst, BST { root: Node::new_node(10, None, None ) });
        assert_ne!(bst, BST { root: Node::new_node(11, None, None ) });
        assert_ne!(bst, BST { root: Node::new_node(11, Node::new_node(10, None, None), None ) });
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
                root: Node::new_node(
                    10,
                    Node::new_node(5, None, None),
                    Node::new_node(
                        15,
                        Node::new_node(
                            12,
                            Node::new_node(
                                11,
                                None,
                                None,
                            ),
                            Node::new_node(13, None, None),
                        ),
                        Node::new_node(
                            18,
                            Node::new_node(17, None, None),
                            Node::new_node(19, None, None),
                        ),
                    ),
                )
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
                root: Node::new_node(
                    12,
                    None,
                    Node::new_node(
                        15,
                        Node::new_node(
                            13,
                            None,
                            None,
                        ),
                        Node::new_node(
                            19,
                            Node::new_node(
                                17,
                                None,
                                None,
                            ),
                            None,
                        ),
                    ),
                )
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

    #[test]
    fn left_rotation_of_root() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(100);
        bst.insert(50);
        bst.insert(200);
        bst.insert(25);
        bst.insert(75);
        bst.insert(150);
        bst.insert(250);
        bst.insert(120);
        bst.insert(170);
        bst.insert(220);
        bst.insert(270);
        bst.insert(210);
        bst.insert(230);
        bst.insert(260);
        bst.insert(280);

        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    100,
                    Node::new_node(
                        50,
                        Node::new_node(
                            25,
                            None,
                            None,
                        ),
                        Node::new_node(
                            75,
                            None,
                            None,
                        ),
                    ),
                    Node::new_node(
                        200,
                        Node::new_node(
                            150,
                            Node::new_node(
                                120,
                                None,
                                None,
                            ),
                            Node::new_node(
                                170,
                                None,
                                None,
                            ),
                        ),
                        Node::new_node(
                            250,
                            Node::new_node(
                                220,
                                Node::new_node(
                                    210,
                                    None,
                                    None,
                                ),
                                Node::new_node(
                                    230,
                                    None,
                                    None,
                                ),
                            ),
                            Node::new_node(
                                270,
                                Node::new_node(
                                    260,
                                    None,
                                    None,
                                ),
                                Node::new_node(
                                    280,
                                    None,
                                    None,
                                ),
                            ),
                        ),
                    ),
                )
            }
        );

        bst.left_rotation_of_root();

        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    200,
                    Node::new_node(
                        100,
                        Node::new_node(
                            50,
                            Node::new_node(
                                25,
                                None,
                                None,
                            ),
                            Node::new_node(
                                75,
                                None,
                                None,
                            ),
                        ),
                        Node::new_node(
                            150,
                            Node::new_node(
                                120,
                                None,
                                None,
                            ),
                            Node::new_node(
                                170,
                                None,
                                None,
                            ),
                        )
                    ),
                    Node::new_node(
                        250,
                        Node::new_node(
                            220,
                            Node::new_node(
                                210,
                                None,
                                None,
                            ),
                            Node::new_node(
                                230,
                                None,
                                None,
                            ),
                        ),
                        Node::new_node(
                            270,
                            Node::new_node(
                                260,
                                None,
                                None,
                            ),
                            Node::new_node(
                                280,
                                None,
                                None,
                            ),
                        ),
                    )
                )
            }
        );
    }

    #[test]
    fn right_rotation_of_root() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(100);
        bst.insert(50);
        bst.insert(20);
        assert_eq!(
            bst,
            BST { root: Node::new_node(100, Node::new_node(50, Node::new_node(20, None, None), None), None) }
        );
        bst.right_rotation_of_root();
        assert_eq!(
            bst,
            BST { root: Node::new_node(50, Node::new_node(20, None, None), Node::new_node(100, None, None)) }
        );
    }
}
