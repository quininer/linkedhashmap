pub mod linkedlist;

use std::borrow::Borrow;
use std::hash::{ Hash, BuildHasher };
use std::collections::{ HashMap, hash_map::RandomState };
use linkedlist::{ LinkedList, NodeSlab };


pub struct LinkedHashMap<K, V, S = RandomState> {
    slab: NodeSlab<V>,
    list: LinkedList,
    map: HashMap<K, usize, S>
}

impl<K, V, S> LinkedHashMap<K, V, S>
where
    K: Hash + Eq + Clone,
    S: BuildHasher
{
    pub fn new() -> LinkedHashMap<K, V> {
        LinkedHashMap {
            slab: NodeSlab::new(),
            list: LinkedList::new(),
            map: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.list.push(&mut self.slab, value);
        let index = self.map.insert(key, index)?;
        self.list.remove(&mut self.slab, index)
    }

    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq
    {
        let index = *self.map.get(key)?;
        self.slab.get(index)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq
    {
        let index = *self.map.get(key)?;
        self.slab.get_mut(index)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq
    {
        let index = self.map.remove(key)?;
        self.list.remove(&mut self.slab, index)
    }
}
