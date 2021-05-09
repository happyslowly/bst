use std::cmp::Ordering::{Equal, Greater, Less};

type Link<T> = Option<Box<BTNode<T>>>;

#[derive(Debug)]
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Link<T>,
}

#[derive(Debug)]
struct BTNode<T>
where
    T: Ord,
{
    item: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BTNode<T>
where
    T: Ord,
{
    fn new(item: T) -> Self {
        BTNode {
            item,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn search(&self, item: &T) -> Option<&T> {
        let mut p = &self.root;
        while let Some(node) = p {
            match item.cmp(&node.item) {
                Equal => return Some(&node.item),
                Less => p = &node.left,
                Greater => p = &node.right,
            }
        }
        None
    }

    pub fn insert(&mut self, item: T) {
        let new_node = Box::new(BTNode::new(item));
        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        let mut p = &mut self.root;
        while let Some(node) = p {
            match new_node.item.cmp(&node.item) {
                Equal => return,
                Less => p = &mut node.left,
                Greater => p = &mut node.right,
            }
        }
        *p = Some(new_node);
    }

    pub fn iter(&self) -> InOrderIter<T> {
        let mut iter = InOrderIter(Vec::new());
        iter.push_left(&self.root);
        iter
    }
}

pub struct InOrderIter<'a, T>(Vec<&'a BTNode<T>>)
where
    T: Ord;

impl<'a, T> InOrderIter<'a, T>
where
    T: Ord,
{
    fn push_left(&mut self, mut link: &'a Link<T>) {
        while let Some(ref n) = link {
            self.0.push(n);
            link = &n.left;
        }
    }
}

impl<'a, T> Iterator for InOrderIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.0.pop() {
            let item = Some(&n.item);
            self.push_left(&n.right);
            item
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = BinarySearchTree::new();

        tree.insert(4);
        tree.insert(2);
        tree.insert(5);
        tree.insert(1);
        tree.insert(3);

        assert_eq!(vec![&1, &2, &3, &4, &5], tree.iter().collect::<Vec<&i32>>());
    }
}
