use std::cmp::Ordering::{Equal, Greater, Less};

type Link<K, V> = Option<Box<BTNode<K, V>>>;

#[derive(Debug)]
pub struct BinarySearchTree<K, V>
where
    K: Ord,
{
    root: Link<K, V>,
    size: usize,
}

#[derive(Debug)]
struct BTNode<K, V>
where
    K: Ord,
{
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K, V> BTNode<K, V>
where
    K: Ord,
{
    fn new(key: K, value: V) -> Self {
        BTNode {
            key,
            value,
            left: None,
            right: None,
        }
    }
}

impl<K, V> BinarySearchTree<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    pub fn search(&self, key: &K) -> Option<(&K, &V)> {
        let mut p = &self.root;
        while let Some(node) = p {
            match key.cmp(&node.key) {
                Equal => return Some((&node.key, &node.value)),
                Less => p = &node.left,
                Greater => p = &node.right,
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut p = &mut self.root;
        while let Some(node) = p {
            match key.cmp(&node.key) {
                Equal => {
                    node.value = value;
                    return;
                }
                Less => p = &mut node.left,
                Greater => p = &mut node.right,
            }
        }
        *p = Some(Box::new(BTNode::new(key, value)));
        self.size += 1;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> InOrderIter<'_, K, V> {
        let mut iter = InOrderIter(Vec::new());
        iter.push_left(&self.root);
        iter
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { iter: self.iter() }
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values { iter: self.iter() }
    }
}

pub struct InOrderIter<'a, K, V>(Vec<&'a BTNode<K, V>>)
where
    K: Ord;

impl<'a, K, V> InOrderIter<'a, K, V>
where
    K: Ord,
{
    fn push_left(&mut self, mut link: &'a Link<K, V>) {
        while let Some(ref n) = link {
            self.0.push(n);
            link = &n.left;
        }
    }
}

impl<'a, K, V> Iterator for InOrderIter<'a, K, V>
where
    K: Ord,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.0.pop() {
            let item = Some((&n.key, &n.value));
            self.push_left(&n.right);
            item
        } else {
            None
        }
    }
}

pub struct Keys<'a, K, V>
where
    K: Ord,
{
    iter: InOrderIter<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V>
where
    K: Ord,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

pub struct Values<'a, K, V>
where
    K: Ord,
{
    iter: InOrderIter<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V>
where
    K: Ord,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = BinarySearchTree::new();

        tree.insert(4, true);
        tree.insert(2, true);
        tree.insert(5, true);
        tree.insert(1, true);
        tree.insert(3, true);

        assert_eq!(5, tree.size());

        assert_eq!(
            vec![
                (&1, &true),
                (&2, &true),
                (&3, &true),
                (&4, &true),
                (&5, &true)
            ],
            tree.iter().collect::<Vec<(&i32, &bool)>>()
        );
    }
}
