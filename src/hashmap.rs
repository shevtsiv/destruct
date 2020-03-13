use crate::linked_list::LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashMap<K: Hash + PartialEq, V: PartialEq> {
    buckets: Vec<LinkedList<Entry<K, V>>>,
}

#[derive(PartialEq)]
struct Entry<K: Hash + PartialEq, V: PartialEq> {
    key: K,
    value: V,
}

impl<K: Hash + PartialEq, V: PartialEq> HashMap<K, V> {
    pub fn put(&mut self, key: K, value: V) {
        let index = self.key_to_index(&key);
        let entry_list = &mut self.buckets[index];
        if let Some(entry) = entry_list.iter_mut().find(|node| node.key == key) {
            entry.value = value;
        } else {
            let new_entry = Entry { key, value };
            entry_list.add(new_entry);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.key_to_index(&key);
        self.buckets[index]
            .iter()
            .find(|entry| entry.key == *key)
            .map(|entry| &entry.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.key_to_index(key);
        self.buckets[index]
            .delete_match(|entry| entry.key == *key)
            .map(|entry| entry.value)
    }

    pub fn new() -> Self {
        let initial_capacity = 32;
        let mut vec = Vec::with_capacity(initial_capacity);
        for i in 0..initial_capacity - 1 {
            vec.insert(i, LinkedList::new())
        }
        HashMap { buckets: vec }
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(initial_capacity);
        for i in 0..initial_capacity {
            vec.insert(i, LinkedList::new())
        }
        HashMap { buckets: vec }
    }

    fn key_to_index(&self, key: &K) -> usize {
        self.hash_key(key) % self.buckets.len()
    }

    fn hash_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::hashmap::HashMap;

    #[test]
    fn put_get() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.put(1, 1);
        map.put(2, 3);
        map.put(4, 5);
        assert_eq!(*map.get(&1).unwrap(), 1);
        assert_eq!(*map.get(&2).unwrap(), 3);
        assert_eq!(*map.get(&4).unwrap(), 5);
    }

    #[test]
    fn rewrite_value() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.put(1, 1);
        map.put(2, 3);
        map.put(4, 5);
        assert_eq!(*map.get(&1).unwrap(), 1);
        assert_eq!(*map.get(&2).unwrap(), 3);
        assert_eq!(*map.get(&4).unwrap(), 5);
        map.put(1, 2);
        assert_eq!(*map.get(&1).unwrap(), 2);
        assert_eq!(*map.get(&2).unwrap(), 3);
        assert_eq!(*map.get(&4).unwrap(), 5);
        map.put(1, 10);
        assert_eq!(*map.get(&1).unwrap(), 10);
        assert_eq!(*map.get(&2).unwrap(), 3);
        assert_eq!(*map.get(&4).unwrap(), 5);
    }

    #[test]
    fn remove() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.put(1, 1);
        map.put(2, 3);
        map.put(4, 5);
        assert_eq!(*map.get(&1).unwrap(), 1);
        assert_eq!(*map.get(&2).unwrap(), 3);
        assert_eq!(*map.get(&4).unwrap(), 5);
        let result = map.remove(&1);
        assert_eq!(result.unwrap(), 1);
        assert_eq!(map.get(&1), None);
        assert_eq!(*map.get(&2).unwrap(), 3);
        assert_eq!(*map.get(&4).unwrap(), 5);
        let second_remove = map.remove(&1);
        assert_eq!(second_remove, None);
        let result = map.remove(&2);
        assert_eq!(result.unwrap(), 3);
        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), None);
        assert_eq!(*map.get(&4).unwrap(), 5);
        let second_remove = map.remove(&2);
        assert_eq!(second_remove, None);
        let result = map.remove(&4);
        assert_eq!(result.unwrap(), 5);
        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&4), None);
    }

    #[test]
    fn single_bucket() {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(1);
        map.put(1, 1);
        map.put(2, 3);
        map.put(4, 5);
        assert_eq!(*map.get(&1).unwrap(), 1);
        assert_eq!(*map.get(&2).unwrap(), 3);
        assert_eq!(*map.get(&4).unwrap(), 5);
        assert_eq!(map.remove(&1).unwrap(), 1);
        assert_eq!(map.remove(&1), None);
        assert_eq!(map.remove(&2).unwrap(), 3);
        assert_eq!(map.remove(&2), None);
        assert_eq!(map.remove(&4).unwrap(), 5);
        assert_eq!(map.remove(&4), None);
    }
}