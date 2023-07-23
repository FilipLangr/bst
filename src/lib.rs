use std::mem;


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
        } else {};
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

// TODO tests
// TODO iterators

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
}
