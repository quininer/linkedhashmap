pub mod linkedlist;

use std::borrow::Borrow;
use std::hash::{ Hash, BuildHasher };
use std::collections::{ HashMap, hash_map::RandomState };
use linkedlist::{ LinkedList, NodeSlab };


pub struct LinkedHashMap<K, V, S = RandomState> {
    slab: NodeSlab<(K, V)>,
    list: LinkedList,
    map: HashMap<K, usize, S>
}

impl<K, V> LinkedHashMap<K, V>
where
    K: Hash + Eq + Clone,
{
    pub fn new() -> LinkedHashMap<K, V> {
        LinkedHashMap {
            slab: NodeSlab::new(),
            list: LinkedList::new(),
            map: HashMap::new()
        }
    }

    pub fn with_capacity(cap: usize) -> LinkedHashMap<K, V> {
        LinkedHashMap {
            slab: NodeSlab::with_capacity(cap),
            list: LinkedList::new(),
            map: HashMap::with_capacity(cap)
        }
    }
}

impl<K, V, S> LinkedHashMap<K, V, S>
where
    K: Hash + Eq + Clone,
    S: BuildHasher
{
    pub fn with_hasher(hash_builder: S) -> Self {
        LinkedHashMap {
            slab: NodeSlab::new(),
            list: LinkedList::new(),
            map: HashMap::with_hasher(hash_builder)
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        LinkedHashMap {
            slab: NodeSlab::with_capacity(capacity),
            list: LinkedList::new(),
            map: HashMap::with_capacity_and_hasher(capacity, hash_builder)
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.slab.reserve(additional);
        self.map.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.slab.shrink_to_fit();
        self.map.shrink_to_fit();
    }

    pub fn clear(&mut self) {
        self.slab.clear();
        self.list = LinkedList::new();
        self.map.clear();
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.list.push(&mut self.slab, (key.clone(), value));
        let index = self.map.insert(key, index)?;
        let (_, value) = self.list.remove(&mut self.slab, index)?;
        Some(value)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq
    {
        let index = *self.map.get(key)?;
        let (_, value) = self.slab.get(index)?;
        Some(value)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq
    {
        let index = *self.map.get(key)?;
        let (_, value) = self.slab.get_mut(index)?;
        Some(value)
    }

    pub fn touch<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq
    {
        let index = *self.map.get(key)?;
        let (_, value) = self.list.touch(&mut self.slab, index)?;
        Some(value)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq
    {
        let index = self.map.remove(key)?;
        let (_, value) = self.list.remove(&mut self.slab, index)?;
        Some(value)
    }

    pub fn pop_front(&mut self) -> Option<(K, V)> {
        let (k, v) = self.list.pop_front(&mut self.slab)?;
        self.map.remove(&k)?;
        Some((k, v))
    }

    pub fn pop_last(&mut self) -> Option<(K, V)> {
        let (k, v) = self.list.pop_last(&mut self.slab)?;
        self.map.remove(&k)?;
        Some((k, v))
    }
}
