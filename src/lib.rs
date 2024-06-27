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
    Node(&'a Node<T>),
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

impl<T: PartialOrd + PartialEq> Default for BST<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T: PartialOrd + PartialEq> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }
    pub fn insert(&mut self, value: T) {
        match self.root.take() {
            None => self.root = Node::new_node(value, None, None),
            Some(root_node) => self.root = root_node.insert(value),
        };
    }
    pub fn contains(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.contains(value))
    }
    pub fn delete(&mut self, value: T) {
        match self.root.take() {
            None => {}
            Some(root) if value == root.value => {
                self.root = Node::delete_by_node( Some(root));
            }
            Some(root) => {
                self.root = root.delete(value);
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
    height: u32,
    balance_factor: i32,
}

impl<T: PartialOrd + PartialEq> Node<T> {
    fn add_left(&mut self, value: T) {
        self.left = Node::new_node(value, None, None)
    }
    fn add_right(&mut self, value: T) {
        self.right = Node::new_node(value, None, None)
    }
    fn new_node(value: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        let mut new_node = Box::new(Node{ value, left, right, height: 0, balance_factor: 0});
        new_node.update_height();
        Some(new_node)
    }
    fn update_height(&mut self) {
        self.height = max(
            self.left.as_ref().map_or(0, |v| v.height + 1),
            self.right.as_ref().map_or(0, |v| v.height + 1)
        );
        self.balance_factor = self.get_balance_factor();
    }
    fn get_balance_factor(&self) -> i32 {
        match (&self.left, &self.right) {
            (None, None) => 0,
            (None, Some(right)) => (right.height as i32) + 1,
            (Some(left), None) => -(left.height as i32) - 1,
            (Some(left), Some(right)) => (right.height as i32) - (left.height as i32)
        }
    }
    fn insert(mut self, value: T) -> Option<Box<Node<T>>> {
        if value < self.value {
            match self.left.take() {
                None => { self.add_left(value) }
                Some(boxed) => { self.left = boxed.insert(value); }
            }
        } else if value > self.value {
            match self.right.take() {
                None => { self.add_right(value) }
                Some(boxed) => { self.right = boxed.insert(value); }
            }
        };
        self.update_height();
        self.rotate()
    }
    fn rotate(mut self) -> Option<Box<Node<T>>> {
        match &self.balance_factor {
            -2 => {
                match self.left.take() {
                    None => panic!("Left can't be None since balance factor is -2"),
                    Some(left) if left.balance_factor > 0 => {
                        self.left = left.left_rotation();
                        self.right_rotation()
                    },
                    Some(left) => {
                        self.left = Some(left);
                        self.right_rotation()
                    },
                }
            },
            -1 => Some(Box::new(self)),
            0 => Some(Box::new(self)),
            1 => Some(Box::new(self)),
            2 => {
                match self.right.take() {
                    None => panic!("Right can't be None since balance factor is 2"),
                    Some(right) if right.balance_factor < 0 => {
                        self.right = right.right_rotation();
                        self.left_rotation()
                    },
                    Some(right) => {
                        self.right = Some(right);
                        self.left_rotation()
                    },
                }
            },
            invalid_bf => panic!("Balance factor should be from interval [-2, 2], but is {}", invalid_bf)
        }
    }
    fn contains(&self, value: T) -> bool {
        if value == self.value { true } else if value < self.value {
            self.left.as_ref().map_or(false, |left| left.contains(value))
        } else {
            self.right.as_ref().map_or(false, |right| right.contains(value))
        }
    }
    fn delete(mut self, value: T) -> Option<Box<Node<T>>> {
        match (self.left.take(), self.right.take()) {
            (left, Some(right)) if value == right.value => {
                self.left = left;
                self.right = Node::delete_by_node( Some(right));
            },
            (Some(left), right) if value == left.value => {
                self.right = right;
                self.left = Node::delete_by_node(Some(left));
            },
            (Some(left), right) if value < self.value => {
                self.right = right;
                self.left = left.delete(value)
            }
            (left, Some(right)) if value > self.value => {
                self.left = left;
                self.right = right.delete(value)
            }
            _ => {}
        };
        self.update_height();
        self.rotate()
    }
    fn delete_by_node(mut node: Option<Box<Node<T>>>) -> Option<Box<Node<T>>>{
        match node.take() {
            None => None,
            Some(to_delete) => match (to_delete.left, to_delete.right) {
                (None, None) => None,
                (None, right) => right,
                (left, None) => left,
                (left, Some(mut right)) => {
                    match right.delete_get_leftmost() {
                        None => {
                            right.left = left;
                            right.update_height();
                            right.rotate()
                        }
                        Some(mut new_node) => {
                            right.update_height();
                            let right = right.rotate();
                            new_node.right = right;
                            new_node.left = left;
                            new_node.update_height();
                            new_node.rotate()
                        }
                    }
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
                ret_val.and_then(|mut _n| {
                    _n.update_height();
                    _n.rotate()
                })
            }
        }
    }
    fn left_rotation(mut self) -> Option<Box<Node<T>>>{
        if let Some(mut right) = self.right.take() {
            self.right = right.left.take();
            self.update_height();
            right.left = Some(Box::new(self));
            right.update_height();
            return Some(right)
        }
        None
    }
    fn right_rotation(mut self) -> Option<Box<Node<T>>>{
        if let Some(mut left) = self.left.take() {
            self.left = left.right.take();
            self.update_height();
            left.right = Some(Box::new(self));
            left.update_height();
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
    fn insert_simple() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    10,
                    None,
                    None,
                )
            }
        );
        bst.insert(20);
        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    10,
                    None,
                    Node::new_node(
                        20,
                        None,
                        None,
                    ),
                )
            }
        );
        bst.insert(30);
        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    20,
                    Node::new_node(
                        10,
                        None,
                        None,
                    ),
                    Node::new_node(
                        30,
                        None,
                        None,
                    ),
                )
            }
        );
        bst.insert(40);
        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    20,
                    Node::new_node(
                        10,
                        None,
                        None,
                    ),
                    Node::new_node(
                        30,
                        None,
                        Node::new_node(
                            40,
                            None,
                            None,
                        )
                    ),
                )
            }
        )
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
                    12,
                    Node::new_node(
                        10,
                        Node::new_node(5, None, None),
                        Node::new_node(11, None, None),
                    ),
                    Node::new_node(
                        15,
                        Node::new_node(
                            13,
                            None,
                            None,
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
                    15,
                    Node::new_node(
                        12,
                        None,
                        Node::new_node(
                            13,
                            None,
                            None,
                        ),
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
    fn delete_root() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.insert(20);
        bst.insert(9);
        bst.insert(8);
        bst.insert(7);

        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    10,
                    Node::new_node(
                        8,
                        Node::new_node(
                            7,
                            None,
                            None
                        ),
                        Node::new_node(
                            9,
                            None,
                            None
                        ),
                    ),
                    Node::new_node(
                        20,
                        None,
                        None,
                    ),
                )
            }
        );

        assert_eq!(bst.root.as_ref().unwrap().height, 2);
        assert_eq!(bst.root.as_ref().unwrap().balance_factor, -1);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().height, 1);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().balance_factor, 0);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().height, 0);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().balance_factor, 0);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().height, 0);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().balance_factor, 0);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().height, 0);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().balance_factor, 0);

        bst.delete(10);

        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    8,
                    Node::new_node(
                        7,
                        None,
                        None,
                    ),
                    Node::new_node(
                        20,
                        Node::new_node(
                            9,
                            None,
                            None
                        ),
                        None
                    ),
                )
            }
        );

        assert_eq!(bst.root.as_ref().unwrap().height, 2);
        assert_eq!(bst.root.as_ref().unwrap().balance_factor, 1);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().height, 0);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().balance_factor, 0);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().height, 1);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().balance_factor, -1);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().left.as_ref().unwrap().height, 0);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().left.as_ref().unwrap().balance_factor, 0);
    }

    #[test]
    fn contains_simple() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(10);
        bst.insert(7);

        assert!(bst.contains(10));
        assert!(bst.contains(5));
        assert!(bst.contains(7));
        assert!(!bst.contains(1313));
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
            BST { root: Node::new_node(50, Node::new_node(20, None, None), Node::new_node(100, None, None)) }
        );
    }

    #[test]
    fn rotate_in_get_leftmost() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(3);
        bst.insert(1);
        bst.insert(5);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(7);

        bst.delete(1);

        assert_eq!(
            bst,
            BST {
                root: Node::new_node(
                    5,
                    Node::new_node(
                        3,
                        Node::new_node(
                            2,
                            None,
                            None,
                        ),
                        Node::new_node(
                            4,
                            None,
                            None,
                        ),
                    ),
                    Node::new_node(
                        6,
                        None,
                        Node::new_node(
                            7,
                            None,
                            None,
                        ),
                    )
                )
            }
        );
    }

    #[test]
    fn test_rotation_of_right_in_delete_by_node() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(6);
        bst.insert(2);
        bst.insert(4);
        bst.insert(9);
        bst.insert(8);
        bst.insert(10);
        bst.insert(1);

        assert_eq!(
            bst,
            BST {root: Node::new_node(
                5,
                Node::new_node(
                    3,
                    Node::new_node(
                        2,
                        Node::new_node(
                            1,
                            None,
                            None
                        ),
                        None
                    ),
                    Node::new_node(
                        4,
                        None,
                        None
                    ),
                ),
                Node::new_node(
                    7,
                    Node::new_node(
                        6,
                        None,
                        None
                    ),
                    Node::new_node(
                        9,
                        Node::new_node(
                            8,
                            None,
                            None
                        ),
                        Node::new_node(
                            10,
                            None,
                            None
                        )
                    )
                )
            )}
        );

        bst.delete(5);

        assert_eq!(
            bst,
            BST {root: Node::new_node(
                6,
                Node::new_node(
                    3,
                    Node::new_node(
                        2,
                        Node::new_node(
                            1,
                            None,
                            None
                        ),
                        None
                    ),
                    Node::new_node(
                        4,
                        None,
                        None
                    ),
                ),
                Node::new_node(
                    9,
                    Node::new_node(
                        7,
                        None,
                        Node::new_node(
                            8,
                            None,
                            None
                        )
                    ),
                    Node::new_node(
                        10,
                        None,
                        None
                    )
                )
            )}
        );


    }

}