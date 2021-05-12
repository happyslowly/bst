use crate::tree::{BinarySearchTree, InOrderIter, Keys, Values};

pub struct TreeSet<T>
where
    T: Ord,
{
    tree: BinarySearchTree<T, bool>,
}

impl<T> TreeSet<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        TreeSet {
            tree: BinarySearchTree::new(),
        }
    }

    pub fn add(&mut self, item: T) {
        self.tree.insert(item, true);
    }

    pub fn contains(&self, item: &T) -> bool {
        self.tree.search(item).is_some()
    }

    pub fn iter(&self) -> Keys<T, bool> {
        self.tree.keys()
    }

    pub fn size(&self) -> usize {
        self.tree.size()
    }
}

pub struct TreeMap<K, V>
where
    K: Ord,
{
    tree: BinarySearchTree<K, V>,
}

impl<K, V> TreeMap<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        TreeMap {
            tree: BinarySearchTree::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.tree.search(key).map(|t| t.1)
    }

    pub fn put(&mut self, key: K, value: V) {
        self.tree.insert(key, value);
    }

    pub fn cotains_key(&self, key: &K) -> bool {
        self.tree.search(key).is_some()
    }

    pub fn iter(&self) -> InOrderIter<K, V> {
        self.tree.iter()
    }

    pub fn keys(&self) -> Keys<K, V> {
        self.tree.keys()
    }

    pub fn values(&self) -> Values<K, V> {
        self.tree.values()
    }

    pub fn size(&self) -> usize {
        self.tree.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_set_works() {
        let mut set = TreeSet::new();

        set.add(1);
        set.add(2);
        set.add(3);
        set.add(1);

        assert_eq!(3, set.size());

        assert_eq!(vec![&1, &2, &3], set.iter().collect::<Vec<&i32>>());
    }

    #[test]
    fn tree_map_works() {
        let mut map = TreeMap::new();

        map.put(1, "foo");
        map.put(2, "bar");
        map.put(3, "hello");
        map.put(3, "world");

        assert_eq!(Some(&"foo"), map.get(&1));
        assert_eq!(None, map.get(&4));
        assert_eq!(3, map.size());

        assert_eq!(vec![&1, &2, &3], map.keys().collect::<Vec<&i32>>());
        assert_eq!(
            vec![&"foo", &"bar", &"world"],
            map.values().collect::<Vec<&&str>>()
        );

        assert_eq!(
            vec![(&1, &"foo"), (&2, &"bar"), (&3, &"world")],
            map.iter().collect::<Vec<(&i32, &&str)>>()
        );
    }
}
