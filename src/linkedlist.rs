use slab::Slab;


pub struct NodeSlab<T>(pub(crate) Slab<Node<T>>);

pub struct LinkedList {
    start: Option<usize>,
    end: Option<usize>
}

pub struct Node<T> {
    pub(crate) value: T,
    prev: Option<usize>,
    next: Option<usize>
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            start: None,
            end: None
        }
    }

    pub fn push<T>(&mut self, NodeSlab(slab): &mut NodeSlab<T>, value: T) -> usize {
        if self.start.is_none() {
            let node = Node {
                value,
                prev: None,
                next: None
            };

            let index = slab.insert(node);
            self.start = Some(index);
            index
        } else {
            let node = Node {
                value,
                prev: self.end,
                next: None
            };

            let index = slab.insert(node);
            self.end = Some(index);
            index
        }
    }

    pub fn pop<T>(&mut self, NodeSlab(slab): &mut NodeSlab<T>) -> Option<T> {
        if let Some(index) = self.end.take() {
            let node = slab.remove(index);
            self.end = node.prev;
            Some(node.value)
        } else {
            Some(slab.remove(self.start.take()?).value)
        }
    }

    pub fn remove<T>(&mut self, NodeSlab(slab): &mut NodeSlab<T>, index: usize) -> Option<T> {
        let node = slab.remove(index);

        if let Some(prev) = node.prev {
            slab[prev].next = node.next;
        } else {
            self.start = node.next;
        }

        if let Some(next) = node.next {
            slab[next].prev = node.prev;
        } else {
            self.end = node.prev;
        }

        Some(node.value)
    }
}

impl<T> NodeSlab<T> {
    pub fn new() -> NodeSlab<T> {
        NodeSlab(Slab::new())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        Some(&self.0.get(index)?.value)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        Some(&mut self.0.get_mut(index)?.value)
    }
}
